use lineas::polynomials::Polynomial;
#[cfg(feature = "plotting")]
use plotters::prelude::*;
#[cfg(feature = "plotting")]
use lineas::polynomials::plotting::PlotArgs;

#[cfg(test)]
mod add {
	use super::*;
	
	#[cfg(test)]
	mod polynomials {
		use super::*;
		
		#[test]
		fn add() {
			let lhs1 = Polynomial::new(vec![1, 2, 3]);
			let lhs2 = Polynomial::new(vec![-5, 3, 10]);
			let rhs = Polynomial::new(vec![-4, 5, 13]);
			assert_eq!(lhs1 + lhs2, rhs)
		}
		
		#[test]
		fn add_zero_last_2() {
			let lhs1 = Polynomial::new(vec![2, 0, 5, -6]);
			let lhs2 = Polynomial::new(vec![-2, 0, 1, 3]);
			let rhs = Polynomial::new(vec![6, -3]);
			assert_eq!(lhs1 + lhs2, rhs)
		}
		
		#[test]
		fn add_assign() {
			let mut lhs1 = Polynomial::new(vec![1, 2, 3]);
			let lhs2 = Polynomial::new(vec![-5, 3, 10]);
			let rhs = Polynomial::new(vec![-4, 5, 13]);
			lhs1 += lhs2;
			assert_eq!(lhs1, rhs)
		}
		
		#[test]
		fn add_assign_zero_last_2() {
			let mut lhs1 = Polynomial::new(vec![2, 0, 5, -6]);
			let lhs2 = Polynomial::new(vec![-2, 0, 1, 3]);
			let rhs = Polynomial::new(vec![6, -3]);
			lhs1 += lhs2;
			assert_eq!(lhs1, rhs)
		}
	}
	
	#[cfg(test)]
	mod literals {
		use super::*;
		
		#[test]
		fn add() {
			let lhs1 = Polynomial::new(vec![1, 2, 3]);
			let lhs2 = 10;
			let rhs = Polynomial::new(vec![1, 2, 13]);
			assert_eq!(lhs1 + lhs2, rhs)
		}
		
		#[test]
		fn add_zero_last_2() {
			let lhs1 = Polynomial::new(vec![0, 0, 5, -6]);
			let lhs2 = 4;
			let rhs = Polynomial::new(vec![5, -2]);
			assert_eq!(lhs1 + lhs2, rhs)
		}
		
		#[test]
		fn add_assign() {
			let mut lhs1 = Polynomial::new(vec![1, 2, 3]);
			let lhs2 = 10;
			let rhs = Polynomial::new(vec![1, 2, 13]);
			lhs1 += lhs2;
			assert_eq!(lhs1, rhs)
		}
		
		#[test]
		fn add_assign_zero_last_2() {
			let mut lhs1 = Polynomial::new(vec![0, 0, 5, -6]);
			let lhs2 = 4;
			let rhs = Polynomial::new(vec![5, -2]);
			lhs1 += lhs2;
			assert_eq!(lhs1, rhs)
		}
	}
}

#[cfg(test)]
mod sub {
	use super::*;
	
	#[cfg(test)]
	mod polynomials {
		use super::*;
		
		#[test]
		fn sub() {
			let lhs1 = Polynomial::new(vec![1, 2, 3]);
			let lhs2 = Polynomial::new(vec![-5, 3, 10]);
			let rhs = Polynomial::new(vec![6, -1, -7]);
			assert_eq!(lhs1 - lhs2, rhs)
		}
		
		#[test]
		fn sub_zero_last_2() {
			let lhs1 = Polynomial::new(vec![2, 0, 5, -6]);
			let lhs2 = Polynomial::new(vec![2, 0, 1, 3]);
			let rhs = Polynomial::new(vec![4, -9]);
			assert_eq!(lhs1 - lhs2, rhs)
		}
		
		#[test]
		fn sub_assign() {
			let mut lhs1 = Polynomial::new(vec![1, 2, 3]);
			let lhs2 = Polynomial::new(vec![-5, 3, 10]);
			let rhs = Polynomial::new(vec![6, -1, -7]);
			lhs1 -= lhs2;
			assert_eq!(lhs1, rhs)
		}
		
		#[test]
		fn sub_assign_zero_last_2() {
			let mut lhs1 = Polynomial::new(vec![2, 0, 5, -6]);
			let lhs2 = Polynomial::new(vec![2, 0, 1, 3]);
			let rhs = Polynomial::new(vec![4, -9]);
			lhs1 -= lhs2;
			assert_eq!(lhs1, rhs)
		}
	}
	
	#[cfg(test)]
	mod literals {
		use super::*;
		
		#[test]
		fn sub() {
			let lhs1 = Polynomial::new(vec![1, 2, 3]);
			let lhs2 = 10;
			let rhs = Polynomial::new(vec![1, 2, -7]);
			assert_eq!(lhs1 - lhs2, rhs)
		}
		
		#[test]
		fn sub_zero_last_2() {
			let lhs1 = Polynomial::new(vec![0, 0, 5, -6]);
			let lhs2 = 4;
			let rhs = Polynomial::new(vec![5, -10]);
			assert_eq!(lhs1 - lhs2, rhs)
		}
		
		#[test]
		fn sub_assign() {
			let mut lhs1 = Polynomial::new(vec![1, 2, 3]);
			let lhs2 = 10;
			let rhs = Polynomial::new(vec![1, 2, -7]);
			lhs1 -= lhs2;
			assert_eq!(lhs1, rhs)
		}
		
		#[test]
		fn sub_assign_zero_last_2() {
			let mut lhs1 = Polynomial::new(vec![0, 0, 5, -6]);
			let lhs2 = 4;
			let rhs = Polynomial::new(vec![5, -10]);
			lhs1 -= lhs2;
			assert_eq!(lhs1, rhs)
		}
	}
}

#[cfg(test)]
mod mult {
	use super::*;
	
	#[cfg(test)]
	mod polynomials {
		use super::*;
		
		#[test]
		fn mult() {
			let lhs1 = Polynomial::new(vec![1, -2, 3]);
			let lhs2 = Polynomial::new(vec![4, 5, -6]);
			let rhs = Polynomial::new(vec![4, -3, -4, 27, -18]);
			assert_eq!(lhs1 * lhs2, rhs)
		}
		
		#[test]
		fn mult_assign() {
			let mut lhs1 = Polynomial::new(vec![2, 5, -6]);
			let lhs2 = Polynomial::new(vec![4, 1, 3]);
			let rhs = Polynomial::new(vec![8, 22, -13, 9, -18]);
			lhs1 *= lhs2;
			assert_eq!(lhs1, rhs)
		}
	}
	
	#[cfg(test)]
	mod literals {
		use super::*;
		
		#[test]
		fn mult() {
			let lhs1 = Polynomial::new(vec![1, 2, 3]);
			let lhs2 = -4;
			let rhs = Polynomial::new(vec![-4, -8, -12]);
			assert_eq!(lhs1 * lhs2, rhs)
		}
		
		#[test]
		fn mult_assign() {
			let mut lhs1 = Polynomial::new(vec![1, 3, -4]);
			let lhs2 = 2;
			let rhs = Polynomial::new(vec![2, 6, -8]);
			lhs1 *= lhs2;
			assert_eq!(lhs1, rhs)
		}
	}
}

#[test]
fn neg() {
	let lhs = Polynomial::new(vec![2, 9, 4, -5]);
	let rhs = Polynomial::new(vec![-2, -9, -4, 5]);
	assert_eq!(-lhs, rhs)
}

#[cfg(feature = "plotting")]
#[test]
fn plot() -> Result<(), Box<dyn std::error::Error>> {
	let root = BitMapBackend::new("test.png", (1024, 768)).into_drawing_area();
	root.fill(&WHITE)?;

	let mut chart = ChartBuilder::on(&root)
		.x_label_area_size(35)
		.y_label_area_size(40)
		.margin(5)
		.caption("A sample graph please help me", ("sans-serif", 50.0).into_font())
		.build_cartesian_2d(-2f32..2f32, -10f32..30f32)?;
	chart.configure_mesh()
		.disable_x_mesh()
		.x_desc("x")
		.y_desc("y")
		.y_label_formatter(&|x| format!("{:.2}", x))
		.draw()?;
	Polynomial::new(vec![3, 0, -3, 9]).dtype::<f32>().plot(&mut chart, PlotArgs { colour: BLUE, legend_precision: 3, points: true, point_resolution: 3, ..Default::default()})?;
	chart
		.configure_series_labels()
		.background_style(&RGBColor(128, 128, 128))
		.draw()?;
	
	root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
	drop(chart);
	Ok(())
}
