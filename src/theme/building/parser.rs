use kasane_logic::Coordinate;
use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use std::fs::File;
use std::io::BufReader;

/// 建物の最小単位の情報
#[derive(Debug, Clone, Default)]
pub struct BuildingInfo {
    // 必ず存在する情報
    pub gml_id: String,
    pub uro_building_id: String,
    pub uro_city_code: String,
    pub class_code: String,

    // 取れない可能性がある情報
    pub measured_height: Option<f64>,
    pub lod1_height_type: Option<i32>,
    pub uro_prefecture_code: Option<String>,
    pub usage_code: Option<i32>,

    // LOD2 のみの Polygon 群
    pub surfaces: Vec<Vec<Coordinate>>,
}

/// 解析中のタグ状態
#[derive(Debug, Clone, Copy, PartialEq)]
enum TargetTag {
    None,
    UroBuildingId,
    UroCity,
    BldgClass,
    MeasuredHeight,
    Lod1HeightType,
    UroPrefecture,
    BldgUsage,
    PosList,
}

/// LOD 状態
#[derive(Debug, Clone, Copy, PartialEq)]
enum LodLevel {
    None,
    Lod0,
    Lod1,
    Lod2,
}

/// Building を 1 件ずつストリームで返すパーサ
pub struct BuildingParser {
    reader: Reader<BufReader<File>>,
    buf: Vec<u8>,

    current_building: BuildingInfo,
    current_tag: TargetTag,
    current_lod: LodLevel,

    current_ring: Vec<Coordinate>,
}

impl BuildingParser {
    pub fn new(file: File) -> Self {
        let mut reader = Reader::from_reader(BufReader::new(file));
        reader.config_mut().trim_text(true);

        Self {
            reader,
            buf: Vec::with_capacity(8192),
            current_building: BuildingInfo::default(),
            current_tag: TargetTag::None,
            current_lod: LodLevel::None,
            current_ring: Vec::new(),
        }
    }

    /// bldg:Building 開始タグから gml:id を抽出
    fn parse_building_attributes(building: &mut BuildingInfo, e: &BytesStart) {
        for attr in e.attributes().flatten() {
            if attr.key.as_ref() == b"gml:id" {
                if let Ok(val) = attr.unescape_value() {
                    building.gml_id = val.into_owned();
                }
            }
        }
    }

    /// Iterator::next() のラッパー
    /// - Some(BuildingInfo): 建物 1 件
    /// - None: EOF
    pub fn read(&mut self) -> impl Iterator<Item = BuildingInfo> {
        self.iter
    }
}

impl Iterator for BuildingParser {
    type Item = BuildingInfo;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.reader.read_event_into(&mut self.buf) {
                Ok(Event::Start(e)) => {
                    match e.name().as_ref() {
                        // 建物開始
                        b"bldg:Building" => {
                            self.current_building = BuildingInfo::default();
                            Self::parse_building_attributes(&mut self.current_building, &e);
                            self.current_lod = LodLevel::None;
                        }

                        // ---- LOD 判定 ----
                        b"bldg:lod0RoofEdge" => self.current_lod = LodLevel::Lod0,
                        b"bldg:lod1Solid" => self.current_lod = LodLevel::Lod1,
                        b"bldg:lod2Solid" | b"bldg:lod2MultiSurface" => {
                            self.current_lod = LodLevel::Lod2
                        }

                        // ---- 属性 ----
                        b"uro:buildingID" => self.current_tag = TargetTag::UroBuildingId,
                        b"uro:city" => self.current_tag = TargetTag::UroCity,
                        b"bldg:class" => self.current_tag = TargetTag::BldgClass,
                        b"bldg:measuredHeight" => self.current_tag = TargetTag::MeasuredHeight,
                        b"uro:lod1HeightType" => self.current_tag = TargetTag::Lod1HeightType,
                        b"uro:prefecture" => self.current_tag = TargetTag::UroPrefecture,
                        b"bldg:usage" => self.current_tag = TargetTag::BldgUsage,

                        // ---- 座標 ----
                        b"gml:posList" => {
                            self.current_ring.clear();
                            self.current_tag = TargetTag::PosList;
                        }

                        _ => {}
                    }
                }

                Ok(Event::Text(e)) => {
                    if self.current_tag == TargetTag::None {
                        // 何もしない
                    } else if let Ok(text) = e.decode() {
                        let s = text.as_ref();
                        match self.current_tag {
                            TargetTag::UroBuildingId => {
                                self.current_building.uro_building_id = s.to_string()
                            }
                            TargetTag::UroCity => {
                                self.current_building.uro_city_code = s.to_string()
                            }
                            TargetTag::BldgClass => {
                                self.current_building.class_code = s.to_string()
                            }
                            TargetTag::MeasuredHeight => {
                                self.current_building.measured_height = s.parse::<f64>().ok();
                            }
                            TargetTag::Lod1HeightType => {
                                self.current_building.lod1_height_type = s.parse::<i32>().ok();
                            }
                            TargetTag::UroPrefecture => {
                                self.current_building.uro_prefecture_code = Some(s.to_string());
                            }
                            TargetTag::BldgUsage => {
                                self.current_building.usage_code = s.parse::<i32>().ok();
                            }
                            TargetTag::PosList => {
                                let nums: Vec<f64> = s
                                    .split_whitespace()
                                    .filter_map(|v| v.parse().ok())
                                    .collect();

                                for c in nums.chunks_exact(3) {
                                    if let Some(coord) = Coordinate::new(c[0], c[1], c[2]) {
                                        self.current_ring.push(coord);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }

                Ok(Event::End(e)) => {
                    match e.name().as_ref() {
                        // Polygon 終了 → LOD2 のみ保存
                        b"gml:Polygon" => {
                            if self.current_lod == LodLevel::Lod2 && !self.current_ring.is_empty() {
                                self.current_building
                                    .surfaces
                                    .push(std::mem::take(&mut self.current_ring));
                            } else {
                                self.current_ring.clear();
                            }
                            self.current_tag = TargetTag::None;
                        }

                        // LOD ブロック終了
                        b"bldg:lod2Solid" | b"bldg:lod2MultiSurface" => {
                            self.current_lod = LodLevel::None;
                        }

                        // 建物終了 → 1 件返す
                        b"bldg:Building" => {
                            self.current_lod = LodLevel::None;
                            self.current_tag = TargetTag::None;
                            return Some(std::mem::take(&mut self.current_building));
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
