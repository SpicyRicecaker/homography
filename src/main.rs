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
use std::process;
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
        // points on the image
        let n: Record = raw_record.deserialize(Some(&headers))?;
        rdr.read_record(&mut raw_record)?;
        // coordinate points
        let p: Record = raw_record.deserialize(Some(&headers))?;

        let mut n_points: VectorOfPoint2f = VectorOfPoint2f::new();

        // add 4 pixels
        n_points.push(Point2f::new(n.x1 as f32, n.y1 as f32));
        n_points.push(Point2f::new(n.x2 as f32, n.y2 as f32));
        n_points.push(Point2f::new(n.x3 as f32, n.y3 as f32));
        n_points.push(Point2f::new(n.x4 as f32, n.y4 as f32));

        let mut p_points: VectorOfPoint2f = VectorOfPoint2f::new();
        // add 4 coordinates
        p_points.push(Point2f::new(p.x1 as f32, p.y1 as f32));
        p_points.push(Point2f::new(p.x2 as f32, p.y2 as f32));
        p_points.push(Point2f::new(p.x3 as f32, p.y3 as f32));
        p_points.push(Point2f::new(p.x4 as f32, p.y4 as f32));

        let mut output: Mat = Mat::default();

        // generate homography matrix using opencv's library
        let h = calib3d::find_homography(&n_points, &p_points, &mut output, 0, 0.001).unwrap();
        dbg!(&h);

        // get the pixel of the runner's shoe
        let image_points = Mat::from_slice_2d(&[[n.x1], [n.y1], [1.]]).unwrap();

        // convert it into a coordinate on the graph
        let res = h * image_points;
        let res: Mat = res.into_result().unwrap().to_mat().unwrap();

        // retrieve the coordinates from the matrix
        let nx: f64 = *res.at_2d(0, 0).unwrap();
        let ny: f64 = *res.at_2d(1, 0).unwrap();
        let nz: f64 = *res.at_2d(2, 0).unwrap();

        // normalize with the z value
        dbg!(nx/nz, ny/nz, nz);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
