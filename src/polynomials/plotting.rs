//! # Plotting module to plot polynomials
//!
//! This adds the [`plot`][crate::polynomials::Polynomial::plot] function and [`PlotArgs`][PlotArgs] struct to plot
//! polynomial functions using the plotters crate
use std::fmt::{Debug, Display};
use conv::{ConvUtil, ValueFrom, ValueInto};
use itertools::Itertools;
use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;

/// Plotting argument struct
///
/// This struct contains the arguments for plotting a polynomial on a graph. This struct implements
/// the [`Default`][PlotArgs::default] trait for default argument values.
#[derive(Clone)]
pub struct PlotArgs<Q> where Q: Color + Copy {
	/// Colour to plot the function
	pub colour: Q,
	/// Display the legend or not
	pub legend: bool,
	/// Coefficient precision in the legend
	pub legend_precision: usize,
	/// Resolution of the plot. This is the number of points per unit.
	///
	/// A resolution of 10 wil plot 10 points per integer space
	pub resolution: usize,
	/// Plot points on the equation
	pub points: bool,
	/// Point resolution. Same idea as [`resolution`][PlotArgs::resolution]
	pub point_resolution: usize,
	/// Size of the points if they're plotted
	pub point_size: i32
}

impl<Q> Default for PlotArgs<Q> where Q: Color + Default + Copy {
	fn default() -> Self {
		Self {
			colour: Q::default(),
			legend: true,
			legend_precision: 1,
			resolution: 10,
			points: false,
			point_resolution: 1,
			point_size: 3
		}
	}
}

impl<L: Copy + Debug + Display> crate::polynomials::Polynomial<L> where L: ValueFrom<isize> + PartialEq + ValueInto<f32>, f32: ValueFrom<L> {
	/// Plot a polynomial on a graph
	///
	/// This requires that you provide the char to plot on and arguments for plotting the chart.
	///
	/// If you're unsure how to build graphs using plotters, I recommend their
	/// [documentation](https://docs.rs/plotters/latest/plotters/)
	pub fn plot<'a, Q: Color + 'a + Copy>(&self, chart: &mut ChartContext<'a, BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>, args: PlotArgs<Q>) -> Result<(), Box<dyn std::error::Error>> {
		let coords = chart.as_coord_spec().get_x_range();
		let (x_min, x_max) = (coords.start.floor() as isize, coords.end.ceil() as isize);
		let e = args.point_resolution;
		let series = chart
			.draw_series(LineSeries::new(
				(x_min * args.resolution as isize..=x_max * args.resolution as isize).map(|x| {
					let x = x as f32 / args.resolution as f32;
					(
						x,
						self.0.iter().enumerate().fold(0f32, |a, (t, e)| a + <L as ValueInto<f32>>::value_into(*e).unwrap() * x.powi((self.0.len() - 1 - t) as i32))
					)
				}),
				&args.colour,
			))?
			.label(format!("y={:.e$}", self));
		if args.legend {
			series.legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &args.colour));
		}
		if args.points {
			chart.draw_series((x_min * args.point_resolution as isize..=x_max * args.point_resolution as isize).map(|x| {
					let x = x as f32 / args.point_resolution as f32;
					Circle::new(
						(
							x,
							self.0.iter().enumerate().fold(0f32, |a, (t, e)| a + <L as ValueInto<f32>>::value_into(*e).unwrap() * x.powi((self.0.len() - 1 - t) as i32))
						),
						args.point_size,
						args.colour.filled().clone()
					)
				})
			)?;
		}
		
		Ok(())
	}
}
