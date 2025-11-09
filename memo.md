# Rust
- Rust 는 강한 정적 타입 시스템을 가진다. 하지만 타입 추론(type inference)도 수행한다.
- 

# Rust 명령어
cargo new 프로젝트명: 새 프로젝트 생성
<br>cargo build: 프로젝트 빌드(디폴트는 디버그 빌드), Cargo.lock 과 target/debug 디렉터리 생성
<br>cargo run: 한번에 프로젝트 빌드 및 실행(실행파일 생성)
<br>cargo check: 컴파일 수행
<br>cargo build --release: 릴리스 빌드 생성(target/release 에 실행파일 생성)
<br>cargo update Cargo.lock 파일을 무시하고 Cargo.toml 에 버전 확인후 해당 버전을 Cargo.lock 에 기록
<br>
<br>둘 이상의 실행 가능한 바이너리를 만들 경우, src 디렉터리 내부에 bin 디렉터리 내부에 소스코드를 저장해야한다
<br>그리고 cargo run 명령에 --bin 옵션을 사용하거나 Cargo.toml 에 기본 실행을 지정해야 한다

## 크레이트(crate)
- Rust 코드 파일들의 모음
- Rust 의 "패키지" 또는 "모듈 단위 프로그램" 이자, 컴파일러가 빌드할 수 있는 최소 단위
- 외부 크레이트 == 라이브러리

## 트레이트(trait)
- 타입이 어떤 "행동"을 해야하는지 정의한 약속(규약)
- Java 의 인터페이스와 비슷함

## 정수형
- i32 는 32bit 정수형(기본값)
- u32 는 부호 없는 32bit 정수형
- i64 는 64bit 정수형

