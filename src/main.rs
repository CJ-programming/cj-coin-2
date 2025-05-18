use warp::Filter;

const DOCKER_BANNER: &str = r#" 
<pre>
                    ##        .            
              ## ## ##       ==            
           ## ## ## ##      ===            
       /""""""""""""""""\___/ ===        
  ~~~ {~~ ~~~~ ~~~ ~~~~ ~~ ~ /  ===- ~~~   
       \______ o          __/            
         \    \        __/             
          \____\______/


Hello from Docker!
</pre>
"#;

#[tokio::main]

async fn main() {

    let hello = warp::path::end().map(||warp::reply::html(DOCKER_BANNER));

    warp::serve(hello).run(([0,0,0,0], 9000)).await;
}