fn main() {
    //라우터 정의 
    let user_routes = Router::new()
        .route("/", get(|| async move {"user"}))
        .route("/login", get(|| async move {"login"}));
    let app = Router::new().nest("/api",user_routes);

    //핸들러 함수 : 요청을 처리(handle) 
    //get, post의 메서드 두번째 매개변수로 넣은 클로저 
    //이때 클로저는 async 
    //풀어서 쓰면 다음과 같음 
    async fn user() -> &'static str { "user" }
    Router::new().route("/",get(user));

    //추출자의 세 가지 종류 
    //1. URL 요소들 
    //2. 요청 본문의 요소들 
    //3. 헤더 

    //경로 파라미터 
    async fn hello() -> String{
        format!("{} : {}",user, user)
    }
    let app=Router::new().route("/path_parameter/:user",get(hello))
}
