use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


#[get("/")]
async fn hello() -> impl Responder {
  
   let html = r#"
   <html>
   <head>
     <title>Magazine</title>
     <style>
       body {
         font-family: Arial, sans-serif;
       }
      .product {
         display: inline-block;
         margin: 10px;
         padding: 10px;
         border: 1px solid #ccc;
         border-radius: 10px;
         box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
       }
      .product img {
         width: 100px;
         height: 100px;
         margin-bottom: 10px;
       }
      .product h2 {
         font-size: 18px;
         margin-bottom: 10px;
       }
      .product p {
         font-size: 14px;
         margin-bottom: 10px;
       }
      .product button {
         background-color: #4CAF50;
         color: #fff;
         padding: 10px 20px;
         border: none;
         border-radius: 5px;
         cursor: pointer;
       }
      .product button:hover {
         background-color: #3e8e41;
       }
       .login-window {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        width: 300px;
        height: 200px;
        background-color: #fff;
        border: 1px solid #ccc;
        border-radius: 10px;
        padding: 20px;
        box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
      }
     .login-window h2 {
        text-align: center;
        margin-bottom: 20px;
      }
     .login-window input {
        width: 100%;
        height: 40px;
        margin-bottom: 20px;
        padding: 10px;
        border: 1px solid #ccc;
        border-radius: 5px;
      }
     .login-window button {
        width: 100%;
        height: 40px;
        background-color: #4CAF50;
        color: #fff;
        padding: 10px;
        border: none;
        border-radius: 5px;
        cursor: pointer;
      }
     .login-window button:hover {
        background-color: #3e8e41;
      }
      .cart {
        position: fixed;
        top: 10px;
        right: 10px;
        background-color: #fff;
        border: 1px solid #ccc;
        border-radius: 10px;
        padding: 10px;
        box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
      }
     .cart h2 {
        margin-top: 0;
      }
     .cart ul {
        list-style: none;
        padding: 0;
        margin: 0;
      }
     .cart li {
        margin-bottom: 10px;
      }
     .cart li span {
        font-weight: bold;
      }
     </style>
   </head>
   <body>
    <div class="login-window">
      <h2>Login</h2>
      <form>
        <input type="text" placeholder="Login" required>
        <input type="password" placeholder="Password" required>
        <button>Login</button>
      </form>
    </div>
  </body>
   <body>
     <h1>Vitrina</h1>
     <div class="products">
       <div class="product">
        <img src="https://yastatic.net/naydex/yandex-search/MSt16e981/f550f1XG/iwyHoKA0kQ3iEW-Rxt26pI8jCfVRwkZWh1OvjNV3004xPtaLv-ix8ygGIzUBQBqbKNZeh6i90KyK9RHtYPwBEh5YwrSE8OVcZ6Rr0EOdOpTfAtwhYNOD9sKCC6kV45bboZ72rQkPJvFuYUUH02cdGtjz-4JsWSHY_HaA" alt="Apple">
         <h2>Apple</h2>
         <p>100/1kg</p>
         <button>Add to Cart</button>
       </div>
       <div class="product">
       <img src="https://yastatic.net/naydex/yandex-search/MSt16e981/f550f1XG/iwyHoKA0kQ3iEW-Rxt26pI8jCfVRwkZWh1OvjNCi9g4hDva7is0Rts3T8wA0Ja_OzaMO5zjo0MzKABSoIPnxB044ojGk8IV8l-R7wHOdOpTfAtwhYNOD9sKCC6kV45bboZ72rQkPJvFuYUUH02cdGtjz-4JsWSHY_HaA" alt="Banana">		
         <h2>Banana</h2>
         <p>150/1kg</p>
         <button>Add to Cart</button>
       </div>
       <div class="product">
         <img src="https://yastatic.net/naydex/yandex-search/MSt17e083/f550f1XG/iwyHoKA0kQ3iEW-Rxt26pI8jCfVRwkZWh1OvjNCn1jsEa6Pbf4hB9i3z0wUklbrraKY7l3i40Jz_xUSIoNwkQi59t4S08KU8Z1Sr8HJJetQfYv1EhUJCJ0JzHv1lIkJu9K42i8kOxvFfsETWY4f9aalzCyJ9jEQYjtaCI" alt="Pear">
         <h2>Pear</h2>
         <p>200/1kg</p>
         <button>Add to Cart</button>
       </div>
     </div>
   </body>
   </html>
    "#;
    HttpResponse::Ok().body(html)
}

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(hello)
        // .service(echo)
        // .route("/hey", web::get().to(manual_hello))
    })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}