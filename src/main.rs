use nalgebra::*;

use std::error::Error;
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

        // See https://www.youtube.com/watch?v=jTCCgxUXhW4
        // [ 2 2 2 2 ] * [h] = [0]
        //      A         X  =  B
        // To solve for X, X=B/A
        // X=B*A^{-1}

        // construct matrix
        let aa: SMatrix<f64, 8, 9> = SMatrix::from_rows(&[
            RowSVector::<f64, 9>::from_vec(vec![
                -n.x1,
                -n.y1,
                -1.,
                0.,
                0.,
                0.,
                n.x1 * p.x1,
                n.y1 * p.x1,
                p.x1,
            ]),
            RowSVector::<f64, 9>::from_vec(vec![
                0.,
                0.,
                0.,
                -n.x1,
                -n.y1,
                -1.,
                n.x1 * p.y1,
                n.y1 * p.y1,
                p.y1,
            ]),
            RowSVector::<f64, 9>::from_vec(vec![
                -n.x2,
                -n.y2,
                -1.,
                0.,
                0.,
                0.,
                n.x2 * p.x2,
                n.y2 * p.x2,
                p.x2,
            ]),
            RowSVector::<f64, 9>::from_vec(vec![
                0.,
                0.,
                0.,
                -n.x2,
                -n.y2,
                -1.,
                n.x2 * p.y2,
                n.y2 * p.y2,
                p.y2,
            ]),
            RowSVector::<f64, 9>::from_vec(vec![
                -n.x3,
                -n.y3,
                -1.,
                0.,
                0.,
                0.,
                n.x3 * p.x3,
                n.y3 * p.x3,
                p.x3,
            ]),
            RowSVector::<f64, 9>::from_vec(vec![
                0.,
                0.,
                0.,
                -n.x3,
                -n.y3,
                -1.,
                n.x3 * p.y3,
                n.y3 * p.y3,
                p.y3,
            ]),
            RowSVector::<f64, 9>::from_vec(vec![
                -n.x4,
                -n.y4,
                -1.,
                0.,
                0.,
                0.,
                n.x4 * p.x4,
                n.y4 * p.x4,
                p.x4,
            ]),
            RowSVector::<f64, 9>::from_vec(vec![
                0.,
                0.,
                0.,
                -n.x4,
                -n.y4,
                -1.,
                n.x4 * p.y4,
                n.y4 * p.y4,
                p.y4,
            ]),
            // RowSVector::<f64, 9>::from_vec(vec![0., 0., 0., 0., 0., 0., 0., 0., 1.]),
        ]);
        // dbg!(aa);
        // let bb: SMatrix<f64, 1, 8> = SMatrix::from_vec(vec![0., 0., 0., 0., 0., 0., 0., 0.]);
        // according to https://medium.com/all-things-about-robotics-and-computer-vision/homography-and-how-to-calculate-it-8abf3a13ddc5,
        // to solve the matrix, SVD is used. How different is that from inverse?
        // let h = bb * aa.try_inverse().unwrap();

        let svd = aa.svd(true, true);

        // **
        // code form https://github.com/dimforge/nalgebra/issues/349#issuecomment-404242773
        // Add an index to all values and sort the tuples, so we know how it was sorted
        // let mut s: Vec<(_, _)> = svd
        //     .singular_values
        //     .into_iter()
        //     .zip(svd.v_t.unwrap().columns(0, 8).iter())
        //     .enumerate()
        //     .map(|(idx, &v)| (v, idx))
        //     .collect();
        // s.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
        // // Unzip the values and order
        // let order: Vec<_> = s.iter().map(|t| t.1).collect();
        // let s: Vec<_> = s.iter().map(|t| t.0).collect();
        // // Reorder u and v using order
        // **


        dbg!(svd.singular_values);
        let h = svd.v_t.unwrap();
        let h = h.column(7);
        // see https://sites.ecse.rpi.edu//~qji/CV/svd_review.pdf


        let homography: Matrix3<f64> =
            Matrix3::from_vec(vec![h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7], 1.]);

        // // now we can get the derived point for the image
        // let image_pos: Matrix3x1<f64> =
        //     Matrix3x1::from_vec(vec![n.pos_runner_x.unwrap(), n.pos_runner_y.unwrap(), 1.]);

        // // DEBUG DEBUG insert original 1 point
        let image_pos: Matrix3x1<f64> = Matrix3x1::from_vec(vec![n.x1, n.y1, 1.]);

        let map_pos = homography * image_pos;

        // dbg!(homography);
        dbg!(image_pos, map_pos);

        // // dbg!(homography);

        // // dbg!(n, p);
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
