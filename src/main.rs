pub mod citygml_to_stid;

pub mod city_gml_to_stid_test;
// use kasane_logic::{function::triangle::triangle, id::coordinates::Point};

// use crate::city_gml_to_stid_test::{citygml_polygon_to_ids, first_building_info};

use crate::citygml_to_stid::bldg::bldg_info;
use crate::citygml_to_stid::brid::brid_info;
use crate::citygml_to_stid::dem::dem_info;
use crate::citygml_to_stid::fld::fld_info;
use crate::citygml_to_stid::frn::frn_info;
use crate::citygml_to_stid::htd::htd_info;
use crate::citygml_to_stid::lsld::lsld_info;
use crate::citygml_to_stid::luse::luse_info;
use crate::citygml_to_stid::tran::tran_info;
use crate::citygml_to_stid::urf::urf_info;



fn main() {
    //引数に並列で動かすスレッド数を指定
    // let _ = bldg_info(1);
    // let _ = brid_info(1);
    // let _ = dem_info(2);
    // let _ = fld_info(1);
    // let _ = frn_info(1);
    // let _ = htd_info(1);
    // let _ = lsld_info(1);
    // let _ = luse_info(1);
    // let _ = tran_info(1);
    // let _ = urf_info(1);
    // // Ok(())

    // let a= Point {
    //     latitude: 36.324935910250225,
    //     longitude: 139.09516767822728,
    //     altitude: 10.0,
    // };

    // let b= Point {
    //     latitude: 35.5880449738478,
    //     longitude: 139.72965784912606,
    //     altitude: 1000.0,
    // };

    // let c= Point {
    //     latitude: 37.58428772730712,
    //     longitude: 139.732310622658,
    //     altitude: 1000.0,
    // };

    // let ids = triangle(20, a, b, c);

    // let ids_new = citygml_polygon_to_ids(16,&[a,b,c]);
    // println!("{:?}",ids_new.iter().map(|stid| stid.to_string()).collect::<Vec<String>>(),);
}
