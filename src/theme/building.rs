use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use std::fs::File;
use std::io::BufReader;

/// 建物の最小単位の情報
#[derive(Debug, Clone, Default)]
pub struct Building {
    //必ず存在する情報
    pub gml_id: String,
    pub uro_building_id: String,
    pub uro_city_code: String,
    pub class_code: String,

    //取れない可能性がある情報
    pub measured_height: Option<f64>,
    pub lod1_height_type: Option<i32>,
    pub uro_prefecture_code: Option<String>,
    pub usage_code: Option<i32>,
}

/// 解析中のタグ状態
#[derive(Debug, Clone, Copy, PartialEq)]
enum TargetTag {
    None,
    UroBuildingId,  // uro:buildingID
    UroCity,        // uro:city
    BldgClass,      // bldg:class
    MeasuredHeight, // bldg:measuredHeight
    Lod1HeightType, // uro:lod1HeightType
    UroPrefecture,  // uro:prefecture
    BldgUsage,      // bldg:usage
}

/// Building を 1 件ずつストリームで返すパーサ
pub struct BuildingParser {
    reader: Reader<BufReader<File>>,
    buf: Vec<u8>,
    current_building: Building,
    current_tag: TargetTag,
}

impl BuildingParser {
    pub fn new(file: File) -> Self {
        let mut reader = Reader::from_reader(BufReader::new(file));
        reader.config_mut().trim_text(true);

        Self {
            reader,
            buf: Vec::with_capacity(8192),
            current_building: Building::default(),
            current_tag: TargetTag::None,
        }
    }

    /// bldg:Building 開始タグから gml:id を抽出
    fn parse_building_attributes(building: &mut Building, e: &BytesStart) {
        for attr in e.attributes().flatten() {
            if attr.key.as_ref() == b"gml:id" {
                if let Ok(val) = attr.unescape_value() {
                    building.gml_id = val.into_owned();
                }
            }
        }
    }
}

impl Iterator for BuildingParser {
    type Item = Building;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.reader.read_event_into(&mut self.buf) {
                Ok(Event::Start(e)) => {
                    match e.name().as_ref() {
                        // 建物開始
                        b"bldg:Building" => {
                            self.current_building = Building::default();
                            Self::parse_building_attributes(&mut self.current_building, &e);
                            self.current_tag = TargetTag::None;
                        }

                        // 対象タグ突入
                        b"uro:buildingID" => self.current_tag = TargetTag::UroBuildingId,
                        b"uro:city" => self.current_tag = TargetTag::UroCity,
                        b"bldg:class" => self.current_tag = TargetTag::BldgClass,
                        b"bldg:measuredHeight" => self.current_tag = TargetTag::MeasuredHeight,
                        b"uro:lod1HeightType" => self.current_tag = TargetTag::Lod1HeightType,
                        b"uro:prefecture" => self.current_tag = TargetTag::UroPrefecture,
                        b"bldg:usage" => self.current_tag = TargetTag::BldgUsage,

                        _ => {}
                    }
                }

                Ok(Event::Text(e)) => {
                    if self.current_tag != TargetTag::None {
                        if let Ok(text) = e.decode() {
                            let s = text.as_ref();
                            match self.current_tag {
                                TargetTag::UroBuildingId => {
                                    self.current_building.uro_building_id = s.to_string();
                                }
                                TargetTag::UroCity => {
                                    self.current_building.uro_city_code = s.to_string();
                                }
                                TargetTag::BldgClass => {
                                    self.current_building.class_code = s.to_string();
                                }
                                TargetTag::MeasuredHeight => {
                                    if let Ok(v) = s.parse::<f64>() {
                                        self.current_building.measured_height = Some(v);
                                    }
                                }
                                TargetTag::Lod1HeightType => {
                                    if let Ok(v) = s.parse::<i32>() {
                                        self.current_building.lod1_height_type = Some(v);
                                    }
                                }
                                TargetTag::UroPrefecture => {
                                    self.current_building.uro_prefecture_code = Some(s.to_string());
                                }
                                TargetTag::BldgUsage => {
                                    if let Ok(v) = s.parse::<i32>() {
                                        self.current_building.usage_code = Some(v);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }

                Ok(Event::End(e)) => {
                    match e.name().as_ref() {
                        // 建物終了 → 1 件返す
                        b"bldg:Building" => {
                            self.current_tag = TargetTag::None;
                            let result = std::mem::take(&mut self.current_building);
                            return Some(result);
                        }
                        _ => {
                            self.current_tag = TargetTag::None;
                        }
                    }
                }

                Ok(Event::Eof) => break,
                Err(e) => {
                    eprintln!("XML parse error: {e}");
                    break;
                }
                _ => {}
            }

            self.buf.clear();
        }

        None
    }
}
