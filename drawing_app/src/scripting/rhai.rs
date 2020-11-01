use rhai::{Engine, EvalAltResult, INT, FLOAT, RegisterFn};

//fn create_point(x: INT, y: INT) -> FLOAT {
//fn create_point<X: Into<f32>,Y: Into<f32>>(x: X, y: Y) -> FLOAT {

// mod crate::cga2d;
use crate::cga2d::*;

#[derive (Default)]
pub struct RunningScript {
	path: std::path::PathBuf,
	engine: Engine,
	initial_state: Vec<Point>,
	current_state: Vec<Point>,
	script_ast: rhai::AST,
}

use std::path::{Path, PathBuf};

#[derive (Default, Copy, Clone, PartialEq)]
struct Point {
	x: FLOAT, 
	y: FLOAT
}

static mut STATE: Vec<Point> = Vec::<Point>::new();
static mut STATE_IDX: usize = 0;


fn represent_point(x: FLOAT, y: FLOAT) -> Conformal2D {
	123.0 * Conformal2D::e1()
}

fn supply_stored_state(x:FLOAT, _:FLOAT) -> Conformal2D {
	unsafe {
		let p = STATE[STATE_IDX];
		STATE_IDX += 1;
		represent_point(p.x,p.x)
	}
}

fn remember_requested_state(x: FLOAT, y: FLOAT) -> Conformal2D {
	unsafe {
		STATE.push(Point{x, y});
	}
	represent_point(x,y)
}


impl RunningScript {
	pub fn new(path: PathBuf) -> RunningScript {

	    let mut engine = Engine::new();

		let mut instance = RunningScript {
			path, engine,
        	..Default::default()
		};

    	unsafe {
    		STATE = Vec::<Point>::new();
    	}
    	instance.engine.register_fn("draggable_point", remember_requested_state);
	
    	instance.modified_script();

    	instance

 //    	let attempt = instance.engine.compile_file(instance.path.clone());

 //    	match attempt {
 //    		Ok(ast) => {
 //    			instance.script_ast = Some(ast);
 //    		},
 //    		Err(e) => {
 //    			println!("Error compiling script: {}", e);
 //    		}

 //    	}
 //    	println!("SCRIPT {:?}",instance.script_ast);

 //    	// attempt to compile script
	//     let err_result: Result<rhai::Dynamic, _> = instance.engine.eval_ast(&instance.script_ast.unwrap());

	//     match err_result {
	//     	Ok(r) => {
	//     		unsafe {
	// 	    		instance.initial_state = STATE.to_owned();
	// 	    		instance.current_state = STATE.to_owned();
	// 	    	}
	//     		println!("result! {}", r.cast::<Conformal2D>());
	//     	},
	//     	Err(e) => println!("Rhai script error: {}", e)
 //    	}
	// //	println!("script {:?}", err_result);

	// 	instance
	}

	fn moop(&mut self) {

	}

	fn dragged_point(&mut self) {
		unsafe {
			STATE = self.current_state.to_owned();
		}
    	self.engine.register_fn("draggable_point", supply_stored_state);

	}

	fn modified_script(&mut self) {
    	let compilation_attempt = self.engine.compile_file(self.path.clone());
println!("modifed");
    	match compilation_attempt {
    		Ok(ast) => {
    			self.script_ast = ast;
		    	println!("SCRIPT {:?}",self.script_ast);

				unsafe {
			   		STATE = Vec::<Point>::new();
			   	}
				self.engine.register_fn("draggable_point", remember_requested_state);

			    let evaluation_attempt: Result<rhai::Dynamic, _> = self.engine.eval_ast(&self.script_ast);
println!("eval'd");
			    match evaluation_attempt {
			    	Ok(r) => {
			    		unsafe {
				    		if STATE != self.initial_state {
				    			self.initial_state = STATE.to_owned();
				    			self.current_state = STATE.to_owned();
				    			
				    		} else {
				    			STATE = self.current_state.to_owned();
						    	self.engine.register_fn("draggable_point", supply_stored_state);
							    let reevaluation_attempt: Result<rhai::Dynamic, _> = self.engine.eval_ast(&self.script_ast);
								match reevaluation_attempt {
							    	Ok(r) => {
							    		println!("script result: {}", r);
							    		// redisplay ok -- request animation frame
							    	},
							    	Err(e) => println!("Rhai script error: {}", e)
							    	// don't redisplay?
							    }

				    		}

				    		// TODO redisplay here
				    		println!("result! {}", r.cast::<Conformal2D>());
				    	}
			    	},
			    	Err(e) => println!("Error evaluating Rhai script: {}", e)
		    	}
    		},
    		Err(e) => {
    			println!("Error compiling Rhai script: {}", e);
    		}

    	}
	}

}

