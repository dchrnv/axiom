#!/usr/bin/env python3
"""
Axiom OS - Comprehensive Benchmark Suite
Полный и честный бенчмарк всех слоев системы

ИСПОЛЬЗОВАНИЕ:
    # Активация виртуального окружения (если используется):
    # source venv/bin/activate  # Linux/Mac
    # venv\\Scripts\\activate   # Windows
    
    # Запуск бенчмарка:
    python benchmarks/comprehensive_benchmark.py

Тестирует:
- Rust Core (прямые FFI вызовы)
- Python FFI (обертки)
- REST API (HTTP endpoints)
- WebSocket (real-time коммуникация)
- Параллельные операции
- Разные масштабы данных
- Статистика (min, max, median, p95, p99, mean, stddev)
- Мониторинг памяти и CPU
"""

import asyncio
import json
import multiprocessing
import platform
import statistics
import subprocess
import sys
import time
from concurrent.futures import ThreadPoolExecutor, ProcessPoolExecutor
from dataclasses import dataclass, asdict
from datetime import datetime
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple
import signal
import os

try:
    import psutil
except ImportError:
    print("⚠️  psutil not installed. Installing...")
    subprocess.check_call([sys.executable, "-m", "pip", "install", "psutil"])
    import psutil

try:
    import requests
except ImportError:
    print("⚠️  requests not installed. Installing...")
    subprocess.check_call([sys.executable, "-m", "pip", "install", "requests"])
    import requests

try:
    import websockets
    WEBSOCKETS_AVAILABLE = True
except ImportError:
    WEBSOCKETS_AVAILABLE = False
    print("⚠️  websockets not installed. WebSocket benchmarks will be skipped.")
    print("   Install with: pip install websockets (or use virtual environment)")


# Конфигурация бенчмарка
BENCHMARK_CONFIG = {
    "scales": [100, 1_000, 10_000, 100_000, 1_000_000],  # Разные масштабы
    "iterations": 5,  # Количество итераций для статистики
    "warmup_iterations": 3,  # Прогрев
    "parallel_workers": [1, 4, 8, 16],  # Параллельные воркеры
    "api_base_url": "http://localhost:8000/api/v1",
    "ws_url": "ws://localhost:8000/ws",
    "grid_radius": 5.0,
    "dimensions": 50,
    "grid_size": 1000,
}


@dataclass
class BenchmarkStats:
    """Статистика бенчмарка."""
    name: str
    count: int
    times: List[float]  # В секундах
    
    @property
    def min(self) -> float:
        return min(self.times) * 1000  # В миллисекундах
    
    @property
    def max(self) -> float:
        return max(self.times) * 1000
    
    @property
    def mean(self) -> float:
        return statistics.mean(self.times) * 1000
    
    @property
    def median(self) -> float:
        return statistics.median(self.times) * 1000
    
    @property
    def stddev(self) -> float:
        if len(self.times) < 2:
            return 0.0
        return statistics.stdev(self.times) * 1000
    
    @property
    def p95(self) -> float:
        sorted_times = sorted(self.times)
        idx = int(len(sorted_times) * 0.95)
        return sorted_times[idx] * 1000
    
    @property
    def p99(self) -> float:
        sorted_times = sorted(self.times)
        idx = int(len(sorted_times) * 0.99)
        return sorted_times[idx] * 1000
    
    @property
    def ops_per_sec(self) -> float:
        return self.count / (self.mean / 1000)
    
    def to_dict(self) -> Dict[str, Any]:
        return {
            "name": self.name,
            "count": self.count,
            "min": round(self.min, 3),
            "max": round(self.max, 3),
            "mean": round(self.mean, 3),
            "median": round(self.median, 3),
            "stddev": round(self.stddev, 3),
            "p95": round(self.p95, 3),
            "p99": round(self.p99, 3),
            "ops_per_sec": round(self.ops_per_sec, 2),
        }


@dataclass
class SystemInfo:
    """Информация о системе."""
    os: str
    python_version: str
    cpu_count_physical: int
    cpu_count_logical: int
    memory_gb: float
    cpu_freq_mhz: float
    
    @classmethod
    def collect(cls) -> "SystemInfo":
        return cls(
            os=f"{platform.system()} {platform.release()}",
            python_version=sys.version.split()[0],
            cpu_count_physical=psutil.cpu_count(logical=False),
            cpu_count_logical=psutil.cpu_count(logical=True),
            memory_gb=round(psutil.virtual_memory().total / (1024**3), 2),
            cpu_freq_mhz=psutil.cpu_freq().current if psutil.cpu_freq() else 0.0,
        )


class BenchmarkRunner:
    """Основной класс для запуска бенчмарков."""
    
    def __init__(self):
        self.results: Dict[str, List[BenchmarkStats]] = {}
        self.system_info = SystemInfo.collect()
        self.api_process: Optional[subprocess.Popen] = None
        self.rust_core_built = False
        self.maturin_available = False
        # Проверяем наличие venv
        self.venv_python = None
        venv_path = Path(__file__).parent.parent / ".venv"
        if venv_path.exists():
            venv_python = venv_path / "bin" / "python"
            if venv_python.exists():
                self.venv_python = str(venv_python)
                print(f"📦 Используется venv: {self.venv_python}")
        
    def print_header(self, text: str):
        """Печать заголовка."""
        print("\n" + "=" * 80)
        print(f"  {text}")
        print("=" * 80)
    
    def print_section(self, text: str):
        """Печать секции."""
        print("\n" + "-" * 80)
        print(f"  {text}")
        print("-" * 80)
    
    def check_and_build_rust_core(self) -> bool:
        """Проверка и сборка Rust Core если нужно."""
        if self.rust_core_built:
            return True
        
        print("\n🔍 Проверка Rust Core...")
        
        # Проверяем доступность FFI (используем venv если доступен)
        python_cmd = [self.venv_python] if self.venv_python else [sys.executable]
        try:
            result = subprocess.run(python_cmd + ["-c", 
                "import sys; sys.path.insert(0, 'src/python'); from axiom import Runtime, Config; "
                "rt = Runtime(Config(grid_size=100, dimensions=10)); "
                "exit(0 if rt.tokens is not None else 1)"],
                cwd=Path(__file__).parent.parent, timeout=10, capture_output=True)
            if result.returncode == 0:
                print("  ✅ Rust Core уже собран и доступен")
                self.rust_core_built = True
                return True
        except:
            pass
        
        # Проверяем maturin
        print("  🔍 Проверка maturin...")
        maturin_cmd = None
        for cmd in ["maturin", "python -m maturin", "cargo maturin"]:
            try:
                result = subprocess.run(cmd.split() + ["--version"], 
                                       capture_output=True, timeout=5)
                if result.returncode == 0:
                    maturin_cmd = cmd.split()
                    self.maturin_available = True
                    print(f"  ✅ Maturin найден: {cmd}")
                    break
            except:
                continue
        
        if not self.maturin_available:
            # Пытаемся установить через pip
            print("  📦 Установка maturin...")
            try:
                subprocess.check_call([sys.executable, "-m", "pip", "install", "--user", "maturin"], 
                                    timeout=60, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
                maturin_cmd = ["maturin"]
                self.maturin_available = True
                print("  ✅ Maturin установлен")
            except Exception as e:
                print(f"  ⚠️  Не удалось установить maturin: {e}")
                print("     Установите вручную: pip install maturin")
                return False
        
        # Собираем Rust Core
        if self.maturin_available:
            print("  🔨 Сборка Rust Core (это может занять несколько минут)...")
            core_rust_path = Path(__file__).parent.parent / "src" / "core_rust"
            
            if not (core_rust_path / "Cargo.toml").exists():
                print(f"  ❌ Cargo.toml не найден в {core_rust_path}")
                return False
            
            try:
                cmd = maturin_cmd + ["develop", "--release", "--features", "python-bindings"]
                print(f"  💻 Выполняется: {' '.join(cmd)}")
                result = subprocess.run(cmd, cwd=core_rust_path, timeout=600, 
                                      stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                                      text=True)
                
                if result.returncode == 0:
                    print("  ✅ Rust Core успешно собран!")
                    self.rust_core_built = True
                    return True
                else:
                    print(f"  ❌ Ошибка сборки:")
                    print(result.stdout[-500:] if len(result.stdout) > 500 else result.stdout)
                    return False
            except subprocess.TimeoutExpired:
                print("  ⚠️  Сборка заняла слишком много времени (>10 минут)")
                return False
            except Exception as e:
                print(f"  ❌ Ошибка при сборке: {e}")
                return False
        
        return False
    
    def measure_time(self, func, *args, **kwargs) -> float:
        """Измерение времени выполнения функции."""
        start = time.perf_counter()
        result = func(*args, **kwargs)
        elapsed = time.perf_counter() - start
        return elapsed, result
    
    def run_with_stats(self, name: str, func, count: int, iterations: int = BENCHMARK_CONFIG["iterations"], *args, **kwargs) -> BenchmarkStats:
        """Запуск функции с сбором статистики."""
        times = []
        
        # Прогрев
        for _ in range(BENCHMARK_CONFIG["warmup_iterations"]):
            try:
                func(*args, **kwargs)
            except Exception:
                pass
        
        # Измерения
        for _ in range(iterations):
            elapsed, _ = self.measure_time(func, *args, **kwargs)
            times.append(elapsed)
        
        stats = BenchmarkStats(name=name, count=count, times=times)
        return stats
    
    # ==================== RUST CORE BENCHMARKS ====================
    
    def benchmark_rust_core(self) -> Dict[str, List[BenchmarkStats]]:
        """Бенчмарк Rust Core (прямые FFI вызовы)."""
        self.print_header("1. RUST CORE BENCHMARKS (Direct FFI)")
        
        results = {}
        
        # Пытаемся собрать Rust Core если нужно
        if not self.check_and_build_rust_core():
            print("  ⚠️  Rust Core не доступен, пропускаем тесты")
            print("     Для сборки: cd src/core_rust && maturin develop --release --features python-bindings")
            return results
        
        try:
            # Используем venv python если доступен
            if self.venv_python:
                # Импортируем через subprocess чтобы использовать venv
                import importlib.util
                spec = importlib.util.spec_from_file_location("axiom", 
                    Path(__file__).parent.parent / "src" / "python" / "axiom" / "__init__.py")
                if spec and spec.loader:
                    axiom_module = importlib.util.module_from_spec(spec)
                    spec.loader.exec_module(axiom_module)
                    Runtime = axiom_module.Runtime
                    Config = axiom_module.Config
                else:
                    sys.path.insert(0, str(Path(__file__).parent.parent / "src" / "python"))
                    from axiom import Runtime, Config
            else:
                sys.path.insert(0, str(Path(__file__).parent.parent / "src" / "python"))
                from axiom import Runtime, Config
            
            # Проверка доступности FFI
            test_rt = Runtime(Config(grid_size=100, dimensions=10))
            if test_rt.tokens is None:
                print("  ❌ Rust Core собран, но FFI не работает (stub mode)")
                print("     Проверьте сборку вручную")
                return results
            
            for scale in BENCHMARK_CONFIG["scales"]:
                if scale > 1_000_000:
                    print(f"⏭️  Пропускаем масштаб {scale:,} (слишком большой для Rust Core)")
                    continue
                
                self.print_section(f"Scale: {scale:,} items")
                
                # 1. Token Creation
                print(f"  📝 Token Creation ({scale:,} tokens)...")
                def create_tokens():
                    rt = Runtime(Config(grid_size=BENCHMARK_CONFIG["grid_size"], 
                                      dimensions=BENCHMARK_CONFIG["dimensions"]))
                    if rt.tokens is None:
                        raise RuntimeError("FFI not available")
                    for i in range(scale):
                        coords = [[float((i + j) % 50) for j in range(BENCHMARK_CONFIG["dimensions"])]]
                        rt.tokens.create({"coordinates": coords, "weight": 1.0})
                    return rt
                
                stats = self.run_with_stats(f"rust_token_create_{scale}", create_tokens, scale)
                results.setdefault("token_creation", []).append(stats)
                print(f"    ✅ {stats.mean:.2f}ms mean ({stats.ops_per_sec:,.0f} ops/s)")
                
                # 2. Token Retrieval
                print(f"  🔍 Token Retrieval ({scale:,} retrievals)...")
                rt = Runtime(Config(grid_size=BENCHMARK_CONFIG["grid_size"], 
                                  dimensions=BENCHMARK_CONFIG["dimensions"]))
                token_ids = []
                for i in range(min(scale, 10000)):  # Ограничиваем для создания
                    coords = [[float((i + j) % 50) for j in range(BENCHMARK_CONFIG["dimensions"])]]
                    token_id = rt.tokens.create({"coordinates": coords, "weight": 1.0})
                    token_ids.append(token_id)
                
                def retrieve_tokens():
                    for token_id in token_ids:
                        _ = rt.tokens.get(token_id)
                
                stats = self.run_with_stats(f"rust_token_retrieve_{len(token_ids)}", retrieve_tokens, len(token_ids))
                results.setdefault("token_retrieval", []).append(stats)
                print(f"    ✅ {stats.mean:.2f}ms mean ({stats.ops_per_sec:,.0f} ops/s)")
                
                # 3. Grid Queries
                print(f"  🗺️  Grid Range Queries (100 queries)...")
                rt = Runtime(Config(grid_size=BENCHMARK_CONFIG["grid_size"], 
                                  dimensions=BENCHMARK_CONFIG["dimensions"]))
                for i in range(min(scale, 1000)):
                    coords = [[float((i + j) % 100) for j in range(BENCHMARK_CONFIG["dimensions"])]]
                    rt.tokens.create({"coordinates": coords, "weight": 1.0})
                
                def grid_queries():
                    for i in range(100):
                        center = tuple([float((i * 13 + j) % 100) for j in range(3)])
                        _ = rt.grid.range_query(center, BENCHMARK_CONFIG["grid_radius"])
                
                stats = self.run_with_stats("rust_grid_range_query_100", grid_queries, 100)
                results.setdefault("grid_queries", []).append(stats)
                print(f"    ✅ {stats.mean:.2f}ms mean ({stats.ops_per_sec:,.0f} ops/s)")
                
        except ImportError as e:
            print(f"  ⚠️  Rust Core не доступен: {e}")
            print("     Установите: cd src/core_rust && maturin develop --release --features python-bindings")
        except RuntimeError as e:
            if "FFI not available" in str(e):
                print(f"  ⚠️  Rust Core FFI не доступен (stub mode)")
                print("     Соберите: cd src/core_rust && maturin develop --release --features python-bindings")
            else:
                raise
        except Exception as e:
            print(f"  ❌ Ошибка в Rust Core бенчмарке: {e}")
            import traceback
            traceback.print_exc()
        
        return results
    
    # ==================== PYTHON FFI BENCHMARKS ====================
    
    def benchmark_python_ffi(self) -> Dict[str, List[BenchmarkStats]]:
        """Бенчмарк Python FFI (обертки над Rust)."""
        self.print_header("2. PYTHON FFI BENCHMARKS (Wrapper Overhead)")
        
        results = {}
        
        try:
            sys.path.insert(0, str(Path(__file__).parent.parent / "src" / "python"))
            from axiom import Runtime, Config
            
            # Проверка доступности
            test_rt = Runtime(Config(grid_size=100, dimensions=10))
            if test_rt.tokens is None:
                print("  ⚠️  Python FFI не доступен (stub mode)")
                return results
            
            for scale in [100, 1_000, 10_000]:
                self.print_section(f"Scale: {scale:,} items")
                
                rt = Runtime(Config(grid_size=BENCHMARK_CONFIG["grid_size"], 
                                  dimensions=BENCHMARK_CONFIG["dimensions"]))
                
                if rt.tokens is None:
                    raise RuntimeError("FFI not available")
                
                # 1. Storage wrapper overhead
                print(f"  📦 Storage Wrapper Overhead ({scale:,} operations)...")
                token_ids = []
                for i in range(scale):
                    coords = [[float((i + j) % 50) for j in range(BENCHMARK_CONFIG["dimensions"])]]
                    token_id = rt.tokens.create({"coordinates": coords, "weight": 1.0})
                    token_ids.append(token_id)
                
                def storage_ops():
                    for token_id in token_ids[:1000]:  # Ограничиваем для скорости
                        _ = rt.tokens.get(token_id)
                
                stats = self.run_with_stats(f"python_ffi_storage_{min(scale, 1000)}", storage_ops, min(scale, 1000))
                results.setdefault("ffi_storage", []).append(stats)
                print(f"    ✅ {stats.mean:.2f}ms mean ({stats.ops_per_sec:,.0f} ops/s)")
                
        except ImportError as e:
            print(f"  ⚠️  Python FFI не доступен: {e}")
        except RuntimeError as e:
            if "FFI not available" in str(e):
                print(f"  ⚠️  Python FFI не доступен (stub mode)")
            else:
                raise
        except Exception as e:
            print(f"  ❌ Ошибка в Python FFI бенчмарке: {e}")
            import traceback
            traceback.print_exc()
        
        return results
    
    # ==================== REST API BENCHMARKS ====================
    
    def start_api_server(self):
        """Запуск API сервера."""
        if self.api_process is not None:
            return
        
        # Проверяем, может сервер уже запущен
        try:
            response = requests.get(f"{BENCHMARK_CONFIG['api_base_url']}/health", timeout=1)
            if response.status_code == 200:
                print("  ℹ️  API сервер уже запущен")
                return
        except:
            pass
        
        print("  🚀 Запуск API сервера...")
        try:
            self.api_process = subprocess.Popen(
                [sys.executable, "-m", "src.api.main"],
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
                preexec_fn=os.setsid,
                cwd=Path(__file__).parent.parent
            )
        except Exception as e:
            print(f"  ⚠️  Не удалось запустить API сервер: {e}")
            raise RuntimeError(f"API сервер не запустился: {e}")
        
        # Ждем запуска
        for i in range(30):
            try:
                response = requests.get(f"{BENCHMARK_CONFIG['api_base_url']}/health", timeout=1)
                if response.status_code == 200:
                    print("  ✅ API сервер запущен")
                    return
            except:
                time.sleep(0.5)
        
        # Если не запустился, но процесс есть - возможно просто медленно
        if self.api_process and self.api_process.poll() is None:
            print("  ⚠️  API сервер запускается медленно, продолжаем...")
            time.sleep(2)
            return
        
        raise RuntimeError("API сервер не запустился за 15 секунд")
    
    def stop_api_server(self):
        """Остановка API сервера."""
        if self.api_process:
            print("  🛑 Остановка API сервера...")
            try:
                os.killpg(os.getpgid(self.api_process.pid), signal.SIGTERM)
                self.api_process.wait(timeout=5)
            except:
                try:
                    self.api_process.kill()
                except:
                    pass
            self.api_process = None
    
    def benchmark_rest_api(self) -> Dict[str, List[BenchmarkStats]]:
        """Бенчмарк REST API."""
        self.print_header("3. REST API BENCHMARKS (HTTP Endpoints)")
        
        results = {}
        
        # Пытаемся запустить API сервер
        try:
            self.start_api_server()
            time.sleep(2)  # Дополнительная пауза
        except RuntimeError as e:
            print(f"  ⚠️  Не удалось запустить API сервер: {e}")
            print("     Проверьте зависимости и попробуйте запустить вручную:")
            print("     python -m src.api.main")
            return results
        
        try:
            
            # Аутентификация (если нужна)
            auth_token = None
            try:
                auth_response = requests.post(
                    f"{BENCHMARK_CONFIG['api_base_url']}/auth/login",
                    json={"username": "developer", "password": "developer123"},
                    timeout=5
                )
                if auth_response.status_code == 200:
                    auth_token = auth_response.json().get("access_token")
            except:
                pass  # Без аутентификации
            
            headers = {}
            if auth_token:
                headers["Authorization"] = f"Bearer {auth_token}"
            
            for scale in [100, 1_000, 10_000]:
                self.print_section(f"Scale: {scale:,} requests")
                
                # 1. Health endpoint
                print(f"  ❤️  Health Endpoint ({scale} requests)...")
                def health_requests():
                    for _ in range(scale):
                        r = requests.get(f"{BENCHMARK_CONFIG['api_base_url']}/health", 
                                       headers=headers, timeout=5)
                        assert r.status_code == 200
                
                stats = self.run_with_stats(f"api_health_{scale}", health_requests, scale, iterations=3)
                results.setdefault("api_health", []).append(stats)
                print(f"    ✅ {stats.mean:.2f}ms mean, p95: {stats.p95:.2f}ms ({stats.ops_per_sec:,.0f} req/s)")
                
                # 2. Token CRUD
                print(f"  📝 Token CRUD ({min(scale, 1000)} operations)...")
                token_ids = []
                
                def create_tokens():
                    nonlocal token_ids
                    token_ids = []
                    for i in range(min(scale, 1000)):
                        r = requests.post(
                            f"{BENCHMARK_CONFIG['api_base_url']}/tokens",
                            json={
                                "coordinates": [[float((i + j) % 50) for j in range(BENCHMARK_CONFIG["dimensions"])]],
                                "weight": 1.0
                            },
                            headers=headers,
                            timeout=10
                        )
                        if r.status_code in [200, 201]:
                            token_ids.append(r.json().get("data", {}).get("id"))
                
                stats = self.run_with_stats(f"api_token_create_{min(scale, 1000)}", create_tokens, min(scale, 1000), iterations=2)
                results.setdefault("api_token_create", []).append(stats)
                print(f"    ✅ {stats.mean:.2f}ms mean ({stats.ops_per_sec:,.0f} ops/s)")
                
                # Get tokens
                if token_ids:
                    print(f"  🔍 Token GET ({len(token_ids)} requests)...")
                    def get_tokens():
                        for token_id in token_ids[:100]:  # Ограничиваем
                            r = requests.get(
                                f"{BENCHMARK_CONFIG['api_base_url']}/tokens/{token_id}",
                                headers=headers,
                                timeout=5
                            )
                            assert r.status_code == 200
                    
                    stats = self.run_with_stats(f"api_token_get_{len(token_ids[:100])}", get_tokens, len(token_ids[:100]), iterations=3)
                    results.setdefault("api_token_get", []).append(stats)
                    print(f"    ✅ {stats.mean:.2f}ms mean ({stats.ops_per_sec:,.0f} ops/s)")
                
        except Exception as e:
            print(f"  ❌ Ошибка в REST API бенчмарке: {e}")
            import traceback
            traceback.print_exc()
        finally:
            self.stop_api_server()
        
        return results
    
    # ==================== WEBSOCKET BENCHMARKS ====================
    
    async def benchmark_websocket_async(self, scale: int) -> BenchmarkStats:
        """Асинхронный бенчмарк WebSocket."""
        try:
            uri = BENCHMARK_CONFIG["ws_url"]
            times = []
            
            for iteration in range(BENCHMARK_CONFIG["iterations"]):
                async with websockets.connect(uri) as ws:
                    # Подписка
                    await ws.send(json.dumps({"action": "subscribe", "channels": ["tokens"]}))
                    await ws.recv()
                    
                    # Измерение latency
                    latencies = []
                    for _ in range(min(scale, 1000)):
                        start = time.perf_counter()
                        await ws.send(json.dumps({"action": "ping"}))
                        response = await ws.recv()
                        elapsed = (time.perf_counter() - start) * 1000  # В миллисекундах
                        latencies.append(elapsed)
                    
                    times.append(statistics.mean(latencies) / 1000)  # В секундах для консистентности
            
            return BenchmarkStats(name=f"ws_latency_{scale}", count=min(scale, 1000), times=times)
        except Exception as e:
            print(f"    ⚠️  WebSocket ошибка: {e}")
            return BenchmarkStats(name=f"ws_latency_{scale}", count=0, times=[0.0])
    
    def benchmark_websocket(self) -> Dict[str, List[BenchmarkStats]]:
        """Бенчмарк WebSocket."""
        self.print_header("4. WEBSOCKET BENCHMARKS (Real-time Communication)")
        
        results = {}
        
        try:
            self.start_api_server()
            time.sleep(2)
            
            for scale in [100, 1_000]:
                self.print_section(f"Scale: {scale:,} messages")
                
                print(f"  🔌 WebSocket Latency ({scale} messages)...")
                stats = asyncio.run(self.benchmark_websocket_async(scale))
                if stats.count > 0:
                    results.setdefault("websocket_latency", []).append(stats)
                    print(f"    ✅ {stats.mean:.2f}ms mean, p95: {stats.p95:.2f}ms")
                else:
                    print(f"    ⚠️  Не удалось выполнить тест")
                
        except Exception as e:
            print(f"  ❌ Ошибка в WebSocket бенчмарке: {e}")
            import traceback
            traceback.print_exc()
        finally:
            self.stop_api_server()
        
        return results
    
    # ==================== PARALLEL BENCHMARKS ====================
    
    def benchmark_parallel(self) -> Dict[str, List[BenchmarkStats]]:
        """Бенчмарк параллельных операций."""
        self.print_header("5. PARALLEL OPERATIONS BENCHMARKS")
        
        results = {}
        
        try:
            sys.path.insert(0, str(Path(__file__).parent.parent / "src" / "python"))
            from axiom import Runtime, Config
            
            # Проверка доступности
            test_rt = Runtime(Config(grid_size=100, dimensions=10))
            if test_rt.tokens is None:
                print("  ⚠️  Параллельные тесты требуют Rust Core FFI")
                return results
            
            scale = 10_000
            self.print_section(f"Scale: {scale:,} operations")
            
            for workers in BENCHMARK_CONFIG["parallel_workers"]:
                print(f"  🔀 Parallel Operations ({workers} workers)...")
                
                def worker_task(worker_id: int):
                    rt = Runtime(Config(grid_size=BENCHMARK_CONFIG["grid_size"], 
                                      dimensions=BENCHMARK_CONFIG["dimensions"]))
                    if rt.tokens is None:
                        raise RuntimeError("FFI not available")
                    ops = 0
                    for i in range(scale // workers):
                        coords = [[float((i * workers + worker_id + j) % 50) for j in range(BENCHMARK_CONFIG["dimensions"])]]
                        rt.tokens.create({"coordinates": coords, "weight": 1.0})
                        ops += 1
                    return ops
                
                def parallel_run():
                    with ThreadPoolExecutor(max_workers=workers) as executor:
                        futures = [executor.submit(worker_task, i) for i in range(workers)]
                        total_ops = sum(future.result() for future in futures)
                    return total_ops
                
                stats = self.run_with_stats(f"parallel_{workers}_workers", parallel_run, scale, iterations=3)
                results.setdefault("parallel", []).append(stats)
                print(f"    ✅ {stats.mean:.2f}ms mean ({stats.ops_per_sec:,.0f} ops/s)")
                
        except RuntimeError as e:
            if "FFI not available" in str(e):
                print(f"  ⚠️  Параллельные тесты требуют Rust Core FFI")
            else:
                raise
        except Exception as e:
            print(f"  ❌ Ошибка в параллельном бенчмарке: {e}")
            import traceback
            traceback.print_exc()
        
        return results
    
    # ==================== MEMORY BENCHMARKS ====================
    
    def benchmark_memory(self) -> Dict[str, Any]:
        """Бенчмарк использования памяти."""
        self.print_header("6. MEMORY USAGE BENCHMARKS")
        
        results = {}
        process = psutil.Process()
        
        try:
            sys.path.insert(0, str(Path(__file__).parent.parent / "src" / "python"))
            from axiom import Runtime, Config
            
            # Проверка доступности
            test_rt = Runtime(Config(grid_size=100, dimensions=10))
            if test_rt.tokens is None:
                print("  ⚠️  Memory тесты требуют Rust Core FFI")
                return results
            
            for scale in [1_000, 10_000, 100_000]:
                print(f"  💾 Memory Usage ({scale:,} tokens)...")
                
                # Базовая память
                mem_before = process.memory_info().rss / (1024**2)  # MB
                
                rt = Runtime(Config(grid_size=BENCHMARK_CONFIG["grid_size"], 
                                  dimensions=BENCHMARK_CONFIG["dimensions"]))
                
                if rt.tokens is None:
                    raise RuntimeError("FFI not available")
                
                # Создание токенов
                for i in range(scale):
                    coords = [[float((i + j) % 50) for j in range(BENCHMARK_CONFIG["dimensions"])]]
                    rt.tokens.create({"coordinates": coords, "weight": 1.0})
                
                mem_after = process.memory_info().rss / (1024**2)  # MB
                mem_used = mem_after - mem_before
                mem_per_token = (mem_used * 1024 * 1024) / scale  # bytes per token
                
                results[f"memory_{scale}"] = {
                    "tokens": scale,
                    "memory_mb": round(mem_used, 2),
                    "bytes_per_token": round(mem_per_token, 2),
                }
                
                print(f"    ✅ {mem_used:.2f} MB ({mem_per_token:.2f} bytes/token)")
                
        except RuntimeError as e:
            if "FFI not available" in str(e):
                print(f"  ⚠️  Memory тесты требуют Rust Core FFI")
            else:
                raise
        except Exception as e:
            print(f"  ❌ Ошибка в memory бенчмарке: {e}")
            import traceback
            traceback.print_exc()
        
        return results
    
    # ==================== RUN ALL ====================
    
    def run_all(self):
        """Запуск всех бенчмарков."""
        print("\n" + "=" * 80)
        print("  AXIOM OS - COMPREHENSIVE BENCHMARK SUITE")
        print("  Полный и честный бенчмарк всех слоев системы")
        print("=" * 80)
        
        print(f"\n📊 System Information:")
        print(f"  OS: {self.system_info.os}")
        print(f"  Python: {self.system_info.python_version}")
        print(f"  CPU: {self.system_info.cpu_count_physical} physical / {self.system_info.cpu_count_logical} logical")
        print(f"  Memory: {self.system_info.memory_gb} GB")
        print(f"  CPU Freq: {self.system_info.cpu_freq_mhz:.0f} MHz")
        
        print(f"\n⚙️  Benchmark Configuration:")
        print(f"  Scales: {BENCHMARK_CONFIG['scales']}")
        print(f"  Iterations: {BENCHMARK_CONFIG['iterations']}")
        print(f"  Parallel Workers: {BENCHMARK_CONFIG['parallel_workers']}")
        
        all_results = {}
        
        # 1. Rust Core
        rust_results = self.benchmark_rust_core()
        all_results.update(rust_results)
        
        # 2. Python FFI
        ffi_results = self.benchmark_python_ffi()
        all_results.update(ffi_results)
        
        # 3. REST API
        api_results = self.benchmark_rest_api()
        all_results.update(api_results)
        
        # 4. WebSocket
        ws_results = self.benchmark_websocket()
        all_results.update(ws_results)
        
        # 5. Parallel
        parallel_results = self.benchmark_parallel()
        all_results.update(parallel_results)
        
        # 6. Memory
        memory_results = self.benchmark_memory()
        all_results["memory"] = memory_results
        
        self.results = all_results
        
        # Генерация отчета
        self.generate_report()
    
    def generate_report(self):
        """Генерация детального отчета."""
        self.print_header("GENERATING REPORT")
        
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        report_path = Path(__file__).parent / f"BENCHMARK_COMPREHENSIVE_{timestamp}.md"
        json_path = Path(__file__).parent / f"BENCHMARK_COMPREHENSIVE_{timestamp}.json"
        
        # Markdown отчет
        report = f"""# Axiom OS - Comprehensive Benchmark Report

**Generated:** {datetime.now().strftime("%Y-%m-%d %H:%M:%S")}
**System:** {self.system_info.os}
**Python:** {self.system_info.python_version}
**CPU:** {self.system_info.cpu_count_physical} physical / {self.system_info.cpu_count_logical} logical cores
**Memory:** {self.system_info.memory_gb} GB

---

## System Configuration

- **OS:** {self.system_info.os}
- **Python:** {self.system_info.python_version}
- **CPU:** {self.system_info.cpu_count_physical} physical cores, {self.system_info.cpu_count_logical} logical cores
- **Memory:** {self.system_info.memory_gb} GB
- **CPU Frequency:** {self.system_info.cpu_freq_mhz:.0f} MHz

---

## Benchmark Results

### 1. Rust Core Performance

"""
        
        # Добавляем результаты
        for category, stats_list in self.results.items():
            if category == "memory":
                continue
            
            if stats_list:
                report += f"\n#### {category.replace('_', ' ').title()}\n\n"
                report += "| Scale | Mean (ms) | Median (ms) | Min (ms) | Max (ms) | p95 (ms) | p99 (ms) | StdDev (ms) | Ops/sec |\n"
                report += "|-------|-----------|-------------|----------|----------|----------|----------|-------------|----------|\n"
                
                for stats in stats_list:
                    report += f"| {stats.count:,} | {stats.mean:.2f} | {stats.median:.2f} | {stats.min:.2f} | {stats.max:.2f} | {stats.p95:.2f} | {stats.p99:.2f} | {stats.stddev:.2f} | {stats.ops_per_sec:,.0f} |\n"
        
        # Memory results
        if "memory" in self.results:
            report += "\n### Memory Usage\n\n"
            report += "| Tokens | Memory (MB) | Bytes/Token |\n"
            report += "|--------|-------------|-------------|\n"
            for key, data in self.results["memory"].items():
                if isinstance(data, dict) and "tokens" in data:
                    report += f"| {data['tokens']:,} | {data['memory_mb']:.2f} | {data['bytes_per_token']:.2f} |\n"
        
        report += "\n---\n\n## Summary\n\n"
        report += "Этот отчет содержит полную статистику всех бенчмарков с минимальными, максимальными, средними, медианными значениями, а также перцентилями p95 и p99.\n"
        
        # Сохранение
        with open(report_path, "w", encoding="utf-8") as f:
            f.write(report)
        
        # JSON
        json_data = {
            "timestamp": datetime.now().isoformat(),
            "system": asdict(self.system_info),
            "config": BENCHMARK_CONFIG,
            "results": {}
        }
        
        for category, stats_list in self.results.items():
            if category == "memory":
                json_data["results"][category] = self.results[category]
            else:
                json_data["results"][category] = [stats.to_dict() for stats in stats_list]
        
        with open(json_path, "w", encoding="utf-8") as f:
            json.dump(json_data, f, indent=2, ensure_ascii=False)
        
        print(f"\n✅ Отчет сохранен:")
        print(f"  📄 Markdown: {report_path}")
        print(f"  📊 JSON: {json_path}")
        print("\n" + "=" * 80)


def main():
    """Главная функция."""
    runner = BenchmarkRunner()
    try:
        runner.run_all()
    except KeyboardInterrupt:
        print("\n\n⚠️  Бенчмарк прерван пользователем")
        runner.stop_api_server()
        sys.exit(1)
    except Exception as e:
        print(f"\n\n❌ Критическая ошибка: {e}")
        import traceback
        traceback.print_exc()
        runner.stop_api_server()
        sys.exit(1)


if __name__ == "__main__":
    main()
