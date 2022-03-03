// use eigenvalues::davidson::Davidson;
// use eigenvalues::{DavidsonCorrection, SpectrumTarget};
// use nalgebra::*;
// use nalgebra::{DMatrix, DVector};
use opencv::calib3d;
use opencv::core::CV_32FC1;
use opencv::core::CV_32FC2;
use opencv::core::CV_64FC1;
use opencv::core::Matx31f;
use opencv::core::Point2f;
use opencv::core::Point3f;
use opencv::core::ToOutputArray;
use opencv::core::Vector;
use opencv::prelude::*;
use opencv::types::VectorOfPoint2f;

use std::error::Error;
use std::f64::INFINITY;
// use std::io;
use std::process;
// use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
struct Record {
    evidence: String,
    time: f64,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x3: f64,
    y3: f64,
    x4: f64,
    y4: f64,
    pos_runner_x: Option<f64>,
    pos_runner_y: Option<f64>,
}

fn example() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_path("data.csv").unwrap();
    let mut raw_record = csv::StringRecord::new();
    let headers = rdr.headers()?.clone();

    while rdr.read_record(&mut raw_record)? {
        let n: Record = raw_record.deserialize(Some(&headers))?;
        rdr.read_record(&mut raw_record)?;
        let p: Record = raw_record.deserialize(Some(&headers))?;

        let mut n_points: VectorOfPoint2f = VectorOfPoint2f::new();

        n_points.push(Point2f::new(n.x1 as f32, n.y1 as f32));
        n_points.push(Point2f::new(n.x2 as f32, n.y2 as f32));
        n_points.push(Point2f::new(n.x3 as f32, n.y3 as f32));
        n_points.push(Point2f::new(n.x4 as f32, n.y4 as f32));

        let mut p_points: VectorOfPoint2f = VectorOfPoint2f::new();
        p_points.push(Point2f::new(p.x1 as f32, p.y1 as f32));
        p_points.push(Point2f::new(p.x2 as f32, p.y2 as f32));
        p_points.push(Point2f::new(p.x3 as f32, p.y3 as f32));
        p_points.push(Point2f::new(p.x4 as f32, p.y4 as f32));

        let mut output: Mat = Mat::default();

        let h = calib3d::find_homography(&n_points, &p_points, &mut output, 1, 0.001).unwrap();
        dbg!(&h);

        // mult h
        let image_points = Mat::from_slice_2d(&[[n.x1], [n.y1], [1.]]).unwrap();

        // let image_points: Mat = Mat::new([n.x1 as f32, n.y1 as f32, 1.]);

        // matrix multiplication is not communicative
        // matrix multiplication is not communicative
        // matrix multiplication is not communicative
        // matrix multiplication is not communicative
        // matrix multiplication is not communicative
        // matrix multiplication is not communicative
        // matrix multiplication is not communicative
        // matrix multiplication is not communicative
        // matrix multiplication is not communicative
        // matrix multiplication is not communicative
        // matrix multiplication is not communicative
        // matrix multiplication is not communicative
        // matrix multiplication is not communicative
        // matrix multiplication is not communicative
        // matrix multiplication is not communicative
        // matrix multiplication is not communicative
        // matrix multiplication is not communicative
        // matrix multiplication is not communicative
        // matrix multiplication is not communicative
        // matrix multiplication is not communicative
        // matrix multiplication is not communicative
        // matrix multiplication is not communicative
        // matrix multiplication is not communicative
        // matrix multiplication is not communicative
        // matrix multiplication is not communicative
        // matrix multiplication is not communicative
        let res = h * image_points;


        let res: Mat = res.into_result().unwrap().to_mat().unwrap();

        dbg!(&res);


        let nx: f64 = *res.at_2d(0, 0).unwrap();
        let ny: f64 = *res.at_2d(1, 0).unwrap();
        let nz: f64 = *res.at_2d(2, 0).unwrap();

        dbg!(nx, ny, nz);

    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }

    // the relationship is
    // let matrix = matrix![

    //     []
    // ];
}
