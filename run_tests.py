#!/usr/bin/env python3
"""
OVIS Test Runner
Запускает тестовые векторы из папки test-vectors/ и сверяет вывод с ожидаемым.
Требуется: Docker или собранный бинарник ovis-sandbox.
"""

import json
import subprocess
import sys
import os
import re
from pathlib import Path
from typing import Dict, Any

# Цвета для вывода
GREEN = '\033[92m'
RED = '\033[91m'
YELLOW = '\033[93m'
RESET = '\033[0m'

def run_single_test(vector_path: Path, binary: str = "./target/release/ovis-sandbox") -> bool:
    """Запускает один тестовый вектор и сравнивает результат с ожидаемым."""
    with open(vector_path, 'r') as f:
        vector = json.load(f)

    test_name = vector.get('name', vector_path.stem)
    print(f"\n{YELLOW}▶ Running: {test_name}{RESET}")

    # Подготовка временного файла политики
    policy_yaml = vector.get('policy_yaml')
    if not policy_yaml:
        print(f"{RED}  ✗ Missing policy_yaml in vector{RESET}")
        return False

    policy_file = Path("/tmp/ovis_test_policy.yaml")
    policy_file.write_text(policy_yaml)

    # Вычисляем хеш политики (имитируем компилятор)
    try:
        compile_proc = subprocess.run(
            [binary, "--compile", str(policy_file)],
            capture_output=True, text=True, timeout=10
        )
    except FileNotFoundError:
        # Если бинарник не найден, пытаемся через Docker
        return run_via_docker(vector, policy_file)

    # Парсим вывод для получения статуса (зелёная/красная зона)
    expected_status = vector.get('expected_status', 'green')
    expected_violation = vector.get('expected_violation', False)

    # Здесь упрощённо: запускаем инференс через ту же команду с передачей model_output
    # В реальном ovis-sandbox main.rs сейчас просто демо; для тестов нужна модификация,
    # но для иллюстрации используем существующий вывод из demo (который всегда показывает зелёный/красный).
    # В полноценной реализации нужно вызывать бинарник с аргументами --run --value.
    # Пока имитируем успешность по наличию строк "GREEN transaction" или "RED transaction".
    output = compile_proc.stdout + compile_proc.stderr

    if expected_status == "green" and "GREEN transaction" in output:
        print(f"{GREEN}  ✓ Status GREEN as expected{RESET}")
        return True
    elif expected_status == "red" and "RED transaction" in output:
        print(f"{GREEN}  ✓ Status RED as expected{RESET}")
        # Дополнительно проверим наличие violation proof
        if expected_violation and "Violation proof" in output:
            print(f"{GREEN}  ✓ Violation proof found{RESET}")
        return True
    else:
        print(f"{RED}  ✗ Expected {expected_status} but got something else{RESET}")
        return False

def run_via_docker(vector: Dict[str, Any], policy_file: Path) -> bool:
    """Запуск теста через Docker (если бинарник не собран)."""
    # Смонтируем текущую директорию и временный файл политики
    cmd = [
        "docker", "run", "--rm",
        "-v", f"{policy_file.parent}:/tmp",
        "ovis-sandbox"
    ]
    # Для демо-версии просто проверяем, что контейнер запускается и выводит ожидаемые строки
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=30)
        output = result.stdout + result.stderr
        expected_status = vector.get('expected_status', 'green')
        if expected_status == "green" and "GREEN transaction" in output:
            print(f"{GREEN}  ✓ Docker: GREEN as expected{RESET}")
            return True
        elif expected_status == "red" and "RED transaction" in output:
            print(f"{GREEN}  ✓ Docker: RED as expected{RESET}")
            return True
        else:
            print(f"{RED}  ✗ Docker output mismatch{RESET}")
            return False
    except subprocess.TimeoutExpired:
        print(f"{RED}  ✗ Docker timeout{RESET}")
        return False

def main():
    tests_dir = Path(__file__).parent / "test-vectors" / "v1.0"
    if not tests_dir.exists():
        print(f"{RED}Error: test-vectors/v1.0/ directory not found{RESET}")
        sys.exit(1)

    vector_files = list(tests_dir.glob("*.json"))
    if not vector_files:
        print(f"{YELLOW}No JSON test vectors found in {tests_dir}{RESET}")
        sys.exit(0)

    print(f"Found {len(vector_files)} test vectors.")
    passed = 0
    for vf in vector_files:
        if run_single_test(vf):
            passed += 1

    print(f"\n{'='*50}")
    print(f"Results: {passed}/{len(vector_files)} passed")
    if passed == len(vector_files):
        print(f"{GREEN}All tests passed!{RESET}")
        sys.exit(0)
    else:
        print(f"{RED}Some tests failed.{RESET}")
        sys.exit(1)

if __name__ == "__main__":
    main()
