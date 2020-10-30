use rhai::{Engine, EvalAltResult, INT, FLOAT, RegisterFn};

//fn create_point(x: INT, y: INT) -> FLOAT {
//fn create_point<X: Into<f32>,Y: Into<f32>>(x: X, y: Y) -> FLOAT {

// mod crate::cga2d;
use crate::cga2d::*;

fn create_point(x: FLOAT, y: FLOAT) -> Conformal2D {
	123.0 * Conformal2D::e1()
}

pub fn load_scripts() {

    let mut engine = Engine::new();
//    	engine.register_fn("create_point", create_point::<INT,INT>)
    	// .register_fn("create_point", create_point<FLOAT,INT>
    	// .register_fn("create_point", create_point<INT,FLOAT>
  		engine.register_fn("create_point", create_point)
    	;


    let err_result: Result<rhai::Dynamic, _> = engine.eval_file("../diagram drawing scripts/test.rhai".into());

    match err_result {
    	Ok(r) => {
    		
    		println!("result! {}", r.cast::<Conformal2D>());
    	},
    	Err(e) => println!("Rhai script error: {}", e)
    }
//	println!("script {:?}", err_result);
}
