# ipinfo

> ipinfo is a command line utility that returns information about the IP address.

## 설치

> 이 프로그램은 Windows를 지원하지 않습니다. \
Windows에서 사용하려면 Windows Subsystem for Linux을 이용하셔야합니다.

ipinfo를 사용하려면, Rust Programming Language의 패키지 매니저인 Cargo가 설치되어있어야 합니다. 만약 이미 설치된 경우, 아래의 명령어를 입력하여 설치를 완료할 수 있습니다.

```zsh
cargo install --git https://github.com/zeroday0619/ipinfo.git
```

Cargo 패키지 매니저 설치는 다음 링크를 참고하세요.

[Cargo 패키지 매니저 설치 방법](https://doc.rust-lang.org/stable/cargo/getting-started/installation.html)

## 사용법

```zsh
ip_info 0.1.0

zeroday0619 <zeroday0619_dev@outlook.com>

Get information about an IP address

USAGE:
    ipinfo --ip <IP> --mode <mode>

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

OPTIONS:
    -i, --ip <IP>        IP address
    -m, --mode <mode>    Mode: [0: asn ,1: country, 2: city, 3: geolocation]

```

### AS Number

```zsh
ipinfo --ip 1.1.1.1 --mode 0
```

### Country

```zsh
ipinfo --ip 1.1.1.1 --mode 1
```

### City

```zsh
ipinfo --ip 1.1.1.1 --mode 2
```

### Geolocation

```zsh
ipinfo --ip 1.1.1.1 --mode 3
```

## 라이선스

이 프로그램은 [MIT 라이선스](./LICENSE)를 따릅니다.
