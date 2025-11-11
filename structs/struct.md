# 구조체(struct)
> 구조체는 여러 값을 묶고 이름을 지어서, 의미 있는 묶음을 정의하는데 사용된다.

## 구조체의 정의 및 인스턴스화
> 구조체를 정의하려면 struct 구조체이름 {} 을 입력한 뒤 중괄호 내부에는 필드(field)라고 부르는 각 구성 요소의 이름 및 타입을 정의한다.<br>
> 구조체의 각 필드에 대한 구체적인 값을 정하여 구조체의 인스턴스(instance)를 생성해야 한다.<br>
> 인스턴스 생성시 먼저 구조체의 이름을 적고, 중괄호를 열어 내부에 필드 이름(key)와 해당 필드에 저장할 값(value)를 키: 값 쌍의 형태로 추가해야 한다.<br>
> 이때, 필드의 순서는 구조체를 정의했을 때와 동일하지 않아도 된다.<br>
> 구조체 내 특정 값은 점(.) 표기로 얻어 올 수 있다. 만약 가변 인스턴스인 경우 특정 필드의 값을 변경할 수도 있다.<br>
> **주의**: 가변성은 해당 인스턴스 전체가 지니고, 일부 필드만 가변으로 만들 수 없다.<br>
> 다른 표현식과 마찬가지로, 함수의 마지막 표현식에 구조체의 새 인스턴스를 생성하는 표현식을 사용해서 해당 인스턴스를 암묵적으로 반환 할 수 있다.<br>
```
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
```
> 위 코드는 매개변수명과 구조체 필드명이 동일하여 반복해서 작성하기 때문에 축약법을 사용하여 줄일 수 있다.

## 필드 초기화 축약법(field init shorthand)
> 변수명과 구조체 필드명이 같을 땐, 필드 초기화 축약법을 사용할 수 있다.<br>
```
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username // 키: 값 형태에서 변수명으로 대체 가능하다
        email // 마찬가지
        sign_in_count: 1,
    }
}
```

## 구조체 업데이트 문법(struct update syntax)
> 인스턴세의 대부분 값을 유지한 채로 몇 개의 값만 변경하여 새 인스턴스를 생성하는 경우, 구조체 업데이트 문법을 사용할 수 있다.
```
// 구조체 업데이트 문법 x
let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"), // 변경이 명시된 필드
        sign_in_count: user1.sign_in_count,
    };
// 구조체 업데이트 문법 o
let user2 = User {
        email: String::from("another@example.com"), // 변경이 명시된 필드
        ..user1
    };
```
> .. 문법은 명시된 필드를 제외한 나머지 필드를 기존 인스턴스 필드값으로 설정한다.<br>
> user1 의 값들과 동일한 값들로 채울려면 ..user1 를 제일 끝에 적어야 하지만, 다른 필드들은 구조체의 정의 내에 있는 필드들의 순서와 상관없이 몇 개든 임의 순서로 적을 수 있다.<br>
> 소유권 이전과 마찬가지로 user1 의 값들로 user2 를 채우면, 필드의 데이터를 이동시키기 때문에 user2 생성후 user1를 더 이상 사용할 수 없다.<br>
> 만약 Copy 트레이트를 구현한 타입만 이동될 경우 기존 인스턴스를 사용할 수 있다. 