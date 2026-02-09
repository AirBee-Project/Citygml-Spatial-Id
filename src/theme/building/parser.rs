use kasane_logic::Coordinate;
use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use std::fs::File;
use std::io::BufReader;

/// 建物の最小単位の情報
#[derive(Debug, Clone, Default)]
pub struct BuildingInfo {
    pub gml_id: String,
    pub uro_building_id: String,
    pub uro_city_code: String,
    pub class_code: String,

    pub measured_height: Option<f64>,
    pub lod1_height_type: Option<i32>,
    pub uro_prefecture_code: Option<String>,
    pub usage_code: Option<i32>,

    /// 採用された Polygon 群（LOD2 優先、なければ LOD1）
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
    Lod1,
    Lod2,
}

/// local-name 判定（prefix 揺れ対策）
fn is_local(name: &[u8], local: &[u8]) -> bool {
    name == local || name.ends_with(local)
}

/// Building を 1 件ずつストリームで返すパーサ
pub struct BuildingParser {
    reader: Reader<BufReader<File>>,
    buf: Vec<u8>,

    current_building: BuildingInfo,
    current_tag: TargetTag,
    current_lod: LodLevel,

    /// LOD 別に一旦保持
    lod1_surfaces: Vec<Vec<Coordinate>>,
    lod2_surfaces: Vec<Vec<Coordinate>>,

    current_ring: Vec<Coordinate>,
    pos_text_buf: String,
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

            lod1_surfaces: Vec::new(),
            lod2_surfaces: Vec::new(),

            current_ring: Vec::new(),
            pos_text_buf: String::new(),
        }
    }

    /// bldg:Building 開始タグから gml:id を抽出
    fn parse_building_attributes(building: &mut BuildingInfo, e: &BytesStart) {
        for attr in e.attributes().flatten() {
            if attr.key.as_ref().ends_with(b"id") {
                if let Ok(val) = attr.unescape_value() {
                    building.gml_id = val.into_owned();
                }
            }
        }
    }

    fn flush_current_polygon(&mut self) {
        if self.pos_text_buf.is_empty() {
            return;
        }

        let nums: Vec<f64> = self
            .pos_text_buf
            .split_whitespace()
            .filter_map(|v| v.parse().ok())
            .collect();

        self.current_ring.clear();
        for c in nums.chunks_exact(3) {
            if let Ok(coord) = Coordinate::new(c[0], c[1], c[2]) {
                self.current_ring.push(coord);
            }
        }

        if self.current_ring.is_empty() {
            return;
        }

        match self.current_lod {
            LodLevel::Lod1 => {
                self.lod1_surfaces
                    .push(std::mem::take(&mut self.current_ring));
            }
            LodLevel::Lod2 => {
                self.lod2_surfaces
                    .push(std::mem::take(&mut self.current_ring));
            }
            _ => {}
        }
    }
}

impl Iterator for BuildingParser {
    type Item = BuildingInfo;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.reader.read_event_into(&mut self.buf) {
                Ok(Event::Start(e)) => match e.name().as_ref() {
                    n if is_local(n, b"Building") => {
                        self.current_building = BuildingInfo::default();
                        Self::parse_building_attributes(&mut self.current_building, &e);

                        self.lod1_surfaces.clear();
                        self.lod2_surfaces.clear();

                        self.current_lod = LodLevel::None;
                        self.current_tag = TargetTag::None;
                    }

                    n if is_local(n, b"lod1Solid") => {
                        self.current_lod = LodLevel::Lod1;
                    }

                    n if is_local(n, b"lod2Solid") || is_local(n, b"lod2MultiSurface") => {
                        self.current_lod = LodLevel::Lod2;
                    }

                    n if is_local(n, b"buildingID") => {
                        self.current_tag = TargetTag::UroBuildingId;
                    }

                    n if is_local(n, b"city") => {
                        self.current_tag = TargetTag::UroCity;
                    }

                    n if is_local(n, b"class") => {
                        self.current_tag = TargetTag::BldgClass;
                    }

                    n if is_local(n, b"measuredHeight") => {
                        self.current_tag = TargetTag::MeasuredHeight;
                    }

                    n if is_local(n, b"lod1HeightType") => {
                        self.current_tag = TargetTag::Lod1HeightType;
                    }

                    n if is_local(n, b"prefecture") => {
                        self.current_tag = TargetTag::UroPrefecture;
                    }

                    n if is_local(n, b"usage") => {
                        self.current_tag = TargetTag::BldgUsage;
                    }

                    n if is_local(n, b"posList") => {
                        self.current_tag = TargetTag::PosList;
                        self.pos_text_buf.clear();
                    }

                    _ => {}
                },

                Ok(Event::Text(e)) => {
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
                                self.current_building.measured_height = s.parse().ok();
                            }
                            TargetTag::Lod1HeightType => {
                                self.current_building.lod1_height_type = s.parse().ok();
                            }
                            TargetTag::UroPrefecture => {
                                self.current_building.uro_prefecture_code = Some(s.to_string());
                            }
                            TargetTag::BldgUsage => {
                                self.current_building.usage_code = s.parse().ok();
                            }
                            TargetTag::PosList => {
                                self.pos_text_buf.push_str(s);
                                self.pos_text_buf.push(' ');
                            }
                            TargetTag::None => {}
                        }
                    }
                }

                Ok(Event::End(e)) => {
                    match e.name().as_ref() {
                        n if is_local(n, b"Polygon") => {
                            self.flush_current_polygon();
                            self.pos_text_buf.clear();
                            self.current_tag = TargetTag::None;
                        }

                        n if is_local(n, b"lod1Solid")
                            || is_local(n, b"lod2Solid")
                            || is_local(n, b"lod2MultiSurface") =>
                        {
                            self.current_lod = LodLevel::None;
                        }

                        n if is_local(n, b"Building") => {
                            // LOD2 優先、なければ LOD1
                            self.current_building.surfaces = if !self.lod2_surfaces.is_empty() {
                                std::mem::take(&mut self.lod2_surfaces)
                            } else {
                                std::mem::take(&mut self.lod1_surfaces)
                            };

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
