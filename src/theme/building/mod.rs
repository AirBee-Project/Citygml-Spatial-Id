use std::{
    cell::{OnceCell, RefCell},
    fs::File,
    path::Path,
};

use kasane_logic::{Solid, TableOnMemory};

use crate::theme::building::parser::{BuildingAttribute, BuildingParser};
pub mod parser;

///1つのbldgファイルを完全なJsonに変換するための機能を提供する型
pub struct Building {
    parser: BuildingParser,
}

impl Building {
    pub fn new(source_file: File) -> Building {
        Building {
            parser: BuildingParser::new(source_file),
        }
    }

    pub fn json() {}

    ///CityGMLから読み取った属性をTableに変換する
    pub fn table(&mut self, z: u8) -> TableOnMemory<BuildingAttribute> {
        //建物の情報を集めたTableを作成する
        let mut table: TableOnMemory<BuildingAttribute> = TableOnMemory::new();

        //パーサーで分解した建物情報を順番に挿入する
        for (value, shape) in &mut self.parser {
            let solid = match Solid::new(shape.surfaces, 0.0) {
                Ok(v) => v,
                Err(e) => {
                    eprint!("{}", e);
                    continue;
                }
            };

            let ids = solid.single_ids(z).unwrap();

            for single_id in ids {
                table.insert(&single_id, &value);
            }
        }

        table
    }
}
