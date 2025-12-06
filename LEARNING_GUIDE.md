# Synchronized Chaos 학습 가이드 📚

Rust를 배우면서 병렬처리와 네트워킹을 익히기 위한 프로젝트 학습 문서.

---

## 🎯 학습 목표

### 핵심 역량
1. **Rust 기초**: ownership, borrowing, lifetime, trait
2. **병렬처리**: 멀티스레딩, lock-free 자료구조, work-stealing
3. **네트워킹**: TCP/UDP, 비동기 I/O, 상태 동기화
4. **게임 네트워킹 패턴**: 서버 권위 모델, 클라이언트 예측, Listen Server

### 게임 클라이언트 개발자로서 채울 약점
- 대규모 병렬처리 경험
- 네트워크 동기화 이해
- "서버는 왜 이렇게 동작하지?"에 대한 감

---

## 📅 일정별 계획

### Phase 1: 기초 시뮬레이션 (Day 1-2)

**목표**: Aizawa attractor 시각화

**학습 내용**:
- Rust 프로젝트 구조 (`cargo new`, 모듈 시스템)
- 기본 문법 (변수, 함수, struct, impl)
- `egui`/`eframe`으로 윈도우 띄우기
- 수치 적분 (Runge-Kutta 4차)

**핵심 코드 연습**:
```rust
// Aizawa attractor 미분방정식
fn aizawa_derivative(p: Vec3, params: &AizawaParams) -> Vec3 {
    let (x, y, z) = (p.x, p.y, p.z);
    Vec3 {
        x: (z - params.b) * x - params.d * y,
        y: params.d * x + (z - params.b) * y,
        z: params.c + params.a * z - z.powi(3) / 3.0 
           - (x * x + y * y) * (1.0 + params.e * z) 
           + params.f * z * x.powi(3),
    }
}
```

**체크포인트**: 
- [x] 파티클 1개가 attractor 궤적을 따라 움직임
- [x] 1000개 이상의 파티클이 화면에 그려짐

---

### Phase 2: Job System 구현 (Day 3-4)

**목표**: 병렬 파티클 업데이트

**학습 내용**:
- `std::thread`, `Arc`, `Mutex`
- `crossbeam` 채널과 deque
- Work-stealing 알고리즘
- Cache locality와 false sharing

**개념 정리**:

| 개념 | 설명 | 왜 중요한가 |
|------|------|------------|
| Work-Stealing | 놀고 있는 스레드가 바쁜 스레드 큐에서 작업을 훔쳐옴 | 로드 밸런싱 자동화 |
| Lock-free Queue | CAS 연산으로 락 없이 동시 접근 | 스레드 경합 최소화 |
| Task Dependency | 작업 간 의존성 그래프 | 순서 보장 필요한 작업 처리 |

**구현 단계**:
1. 단순 스레드 풀 (고정 작업 분배)
2. 작업 큐 추가 (동적 분배)
3. Work-stealing 구현
4. 의존성 처리 (선택)

**참고할 것들**:
- `rayon` 소스코드 (너무 복잡하면 개념만)
- crossbeam의 `deque` 모듈
- 논문: "Scheduling Multithreaded Computations by Work Stealing"

**체크포인트**:
- [ ] 파티클 업데이트가 멀티코어 사용
- [ ] 싱글스레드 대비 성능 향상 측정

---

### Phase 3: 네트워크 기초 (Day 5-6)

**목표**: 고정 호스트로 두 머신 동기화

**학습 내용**:
- `tokio` 비동기 런타임
- `async`/`await` 문법
- TCP 소켓 (연결 지향, 신뢰성)
- `serde` + `bincode` 직렬화

**네트워크 기초 개념**:

```
TCP vs UDP:
┌─────────────────────────────────────────────────┐
│ TCP: 신뢰성 O, 순서 보장 O, 느림                  │
│      → 중요한 상태 동기화에 사용                  │
│                                                 │
│ UDP: 신뢰성 X, 순서 보장 X, 빠름                  │
│      → 실시간 위치 업데이트, 디스커버리에 사용     │
└─────────────────────────────────────────────────┘
```

**동기화 전략**:
- **서버 권위 (Server Authority)**: 호스트의 시뮬레이션이 "진짜"
- **클라이언트 예측 (Client Prediction)**: 로컬에서 미리 계산, 서버 결과로 보정
- **스냅샷 보간 (Snapshot Interpolation)**: 과거 스냅샷 사이를 보간해서 부드럽게

**메시지 프로토콜 설계**:
```rust
enum Message {
    // 연결
    Connect { client_id: u64 },
    Disconnect { client_id: u64 },
    
    // 동기화
    StateSnapshot { tick: u64, particles: Vec<ParticleState> },
    
    // 하트비트
    Ping { timestamp: u64 },
    Pong { timestamp: u64 },
}
```

**체크포인트**:
- [ ] 맥북(호스트)에서 시뮬레이션 실행
- [ ] 윈도우(클라이언트)가 연결해서 같은 화면 표시
- [ ] 네트워크 지연이 있어도 동기화 유지

---

### Phase 4: 자동 디스커버리 (Day 7)

**목표**: 누가 먼저 켜든 자동으로 역할 결정

**학습 내용**:
- UDP 브로드캐스트
- 타임아웃 처리
- 상태 머신 패턴

**디스커버리 흐름**:
```
시작
  │
  ▼
UDP 브로드캐스트 "HOST_QUERY" ──────┐
  │                               │
  │ (2초 대기)                     │
  ▼                               ▼
응답 없음?                      응답 있음?
  │                               │
  ▼                               ▼
호스트 모드로 전환              클라이언트로 연결
```

**체크포인트**:
- [ ] 첫 번째 실행 인스턴스가 자동으로 호스트
- [ ] 두 번째 인스턴스가 자동으로 클라이언트로 연결

---

### Phase 5: Host Migration (확장)

**목표**: 호스트가 꺼져도 시뮬레이션 계속

**학습 내용**:
- 분산 시스템 리더 선출
- 상태 전이 (State Transfer)
- Consistency vs Availability 트레이드오프

**고려사항**:
- Chaotic system이라 상태 전달이 정확해야 함
- 전환 중 화면 끊김 최소화
- 어떤 클라이언트가 새 호스트가 될지 결정

**체크포인트**:
- [ ] 호스트 종료 시 다른 피어가 자동으로 호스트 승계
- [ ] 시뮬레이션 상태가 유지됨

---

## 🔧 개발 환경 설정

### 필수 도구
```bash
# Rust 설치
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 크로스 컴파일 (선택)
rustup target add x86_64-pc-windows-msvc  # 맥에서 윈도우용 빌드
```

### 추천 VSCode 확장
- rust-analyzer
- CodeLLDB (디버깅)
- Even Better TOML

### Cargo.toml 기본 의존성
```toml
[dependencies]
eframe = "0.27"              # GUI
egui = "0.27"
tokio = { version = "1", features = ["full"] }
crossbeam = "0.8"            # 병렬처리
serde = { version = "1", features = ["derive"] }
bincode = "1"                # 바이너리 직렬화
glam = "0.27"                # 수학 (Vec3 등)
```

---

## 📖 참고 자료

### Rust 기초
- [The Rust Book](https://doc.rust-lang.org/book/) - 공식 튜토리얼
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

### 병렬처리
- [Rayon 문서](https://docs.rs/rayon/latest/rayon/)
- [Crossbeam 문서](https://docs.rs/crossbeam/latest/crossbeam/)

### 네트워킹
- [Tokio 튜토리얼](https://tokio.rs/tokio/tutorial)
- [Bevy의 네트워킹 디자인 문서](https://github.com/bevyengine/bevy/discussions) (참고용)

### 게임 네트워킹
- [게임 네트워킹 시리즈 - Gabriel Gambetta](https://www.gabrielgambetta.com/client-server-game-architecture.html)
- [Overwatch GDC Talk - Netcode](https://www.youtube.com/watch?v=W3aieHjyNvw)

---

## 💡 팁

### C++ 개발자를 위한 Rust 매핑

| C++ | Rust | 비고 |
|-----|------|------|
| `std::unique_ptr<T>` | `Box<T>` | 소유권 이동 |
| `std::shared_ptr<T>` | `Arc<T>` | 스레드 안전 참조 카운팅 |
| `std::mutex` | `Mutex<T>` | 데이터와 락이 결합됨! |
| `std::thread` | `std::thread::spawn` | 비슷함 |
| `template<T>` | `<T: Trait>` | 트레이트 바운드 |
| RAII | 동일 | Drop 트레이트 |

### 자주 하는 실수
1. **`&mut` 두 개 동시에 못 씀** - 컴파일러가 막음, 설계를 바꿔야 함
2. **`Arc<Mutex<T>>`가 기본 패턴** - 멀티스레드에서 공유 데이터
3. **`async`는 `tokio::spawn` 안에서** - 런타임 필요

---

## ✅ 최종 체크리스트

프로젝트 완료 시 답할 수 있어야 하는 질문들:

**Rust**
- [ ] Ownership과 borrowing이 왜 필요한가?
- [ ] `Arc<Mutex<T>>`는 언제 쓰는가?
- [ ] `async/await`는 어떻게 동작하는가?

**병렬처리**
- [ ] Work-stealing이 왜 효율적인가?
- [ ] False sharing이 뭐고 어떻게 피하는가?
- [ ] Task dependency는 어떻게 처리하는가?

**네트워킹**
- [ ] TCP와 UDP의 차이와 용도는?
- [ ] 서버 권위 모델이 왜 필요한가?
- [ ] 클라이언트 예측은 어떻게 구현하는가?
- [ ] Host Migration의 어려운 점은?

---

화이팅! 🔥