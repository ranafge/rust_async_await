use std::future::Future;

pub async fn foo() -> u8 {5}

pub fn bar() -> impl Future<Output=u8> {
    async {
        let x  = foo().await;
        x + 5

    }
}
pub async fn borrow_x(x: &u8)-> u8 {
   let x  =  *x;
   x
}
pub async fn good() -> impl Future<Output=u8> {
    async {
        let x  = 5;
       let res =  borrow_x(&x).await;
       println!("{:?}", res);
       0
    }
}

