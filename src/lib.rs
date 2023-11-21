//! # Camera Trap Data Package
//! This crate provides a Rust implementation of the [Camera Trap Data Package](https://camtrap-dp.tdwg.org/data/).
//! The Camera Trap Data Package is a specification for a data package to describe camera trap deployments and media files recorded during deployments.
//! The specification is based on the [Data Package](https://frictionlessdata.io/specs/data-package/) and [Tabular Data Package](https://frictionlessdata.io/specs/tabular-data-package/) specifications.

use bytes::Buf;
use serde::{Deserialize, Serialize};

/// Camera trap placement (deployment).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Deployment {
    /// Unique identifier of the deployment.
    #[serde(rename = "deploymentID")]
    pub deployment_id: String,

    /// Identifier of the deployment location.
    #[serde(rename = "locationID")]
    pub location_id: Option<String>,

    /// Name given to the deployment location.
    #[serde(rename = "locationName")]
    pub location_name: Option<String>,

    /// Latitude of the deployment location in decimal degrees, using the WGS84 datum.
    pub latitude: Option<f64>,

    /// Horizontal distance from the given latitude and longitude describing the smallest circle containing the deployment location. Expressed in meters. Especially relevant when coordinates are rounded to protect sensitive species.
    #[serde(rename = "locationRadius")]
    pub location_radius: Option<f64>,

    /// Date and time at which the deployment was started. Formatted as an ISO 8601 string with timezone designator (YYYY-MM-DDThh:mm:ssZ or YYYY-MM-DDThh:mm:ss±hh:mm).
    #[serde(rename = "deploymentStart")]
    pub deployment_start: chrono::DateTime<chrono::FixedOffset>,

    /// Date and time at which the deployment was ended. Formatted as an ISO 8601 string with timezone designator (YYYY-MM-DDThh:mm:ssZ or YYYY-MM-DDThh:mm:ss±hh:mm).
    #[serde(rename = "deploymentEnd")]
    pub deployment_end: chrono::DateTime<chrono::FixedOffset>,

    /// Name or identifier of the person or organization that deployed the camera.
    #[serde(rename = "setupBy")]
    pub setup_by: Option<String>,

    /// Identifier of the camera used for the deployment (e.g. the camera device serial number).
    #[serde(rename = "cameraID")]
    pub camera_id: Option<String>,

    /// Manufacturer and model of the camera. Formatted as manufacturer-model.
    #[serde(rename = "cameraModel")]
    pub camera_model: Option<String>,

    /// Predefined duration after detection when further activity is ignored. Expressed in seconds.
    #[serde(rename = "cameraDelay")]
    pub camera_delay: Option<f64>,

    /// Height at which the camera was deployed. Expressed in meters. Not to be combined with cameraDepth.
    #[serde(rename = "cameraHeight")]
    pub camera_height: Option<f64>,

    /// Depth at which the camera was deployed. Expressed in meters. Not to be combined with cameraHeight.
    #[serde(rename = "cameraDepth")]
    pub camera_depth: Option<f64>,

    /// Angle at which the camera was deployed in the vertical plane. Expressed in degrees, with -90 facing down, 0 horizontal and 90 facing up.
    #[serde(rename = "cameraAngle")]
    pub camera_angle: Option<f64>,

    /// Angle at which the camera was deployed in the horizontal plane. Expressed in decimal degrees clockwise from north, with values ranging from 0 to 360: 0 = north, 90 = east, 180 = south, 270 = west.
    #[serde(rename = "cameraHeading")]
    pub camera_heading: Option<f64>,

    /// Maximum distance at which the camera can reliably detect activity. Expressed in meters. Typically measured by having a human move in front of the camera.
    #[serde(rename = "detectionDistance")]
    pub detection_distance: Option<f64>,

    /// true if timestamps in the media resource for the deployment are known to have (unsolvable) issues (e.g. unknown timezone, am/pm switch).
    #[serde(rename = "timestampIssues")]
    pub timestamp_issues: Option<bool>,

    /// true if bait was used for the deployment. More information can be provided in tags or comments.
    #[serde(rename = "baitUse")]
    pub bait_use: Option<bool>,

    /// Type of the feature (if any) associated with the deployment.
    #[serde(rename = "featureType")]
    pub feature_type: Option<FeatureType>,

    /// Short characterization of the habitat at the deployment location.
    pub habitat: Option<String>,

    /// Deployment group(s) associated with the deployment. Deployment groups can have a spatial (arrays, grids, clusters), temporal (sessions, seasons, months, years) or other context. Formatted as a pipe (|) separated list for multiple values, with values preferably formatted as key:value pairs.
    #[serde(rename = "deploymentGroups")]
    pub deployment_groups: Option<String>,

    /// Tag(s) associated with the deployment. Formatted as a pipe (|) separated list for multiple values, with values optionally formatted as key:value pairs.
    pub tags: Option<String>,

    /// Comments or notes about the deployment.
    pub comments: Option<String>,
}

/// Type of the feature (if any) associated with the deployment.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum FeatureType {
    #[serde(rename = "roadPaved")]
    RoadPaved,

    #[serde(rename = "roadDirt")]
    RoadDirt,

    #[serde(rename = "trailHiking")]
    TrailHiking,

    #[serde(rename = "trailGame")]
    TrailGame,

    #[serde(rename = "roadUnderpass")]
    RoadUnderpass,

    #[serde(rename = "roadOverpass")]
    RoadOverpass,

    #[serde(rename = "roadBridge")]
    RoadBridge,

    #[serde(rename = "culvert")]
    Culvert,

    #[serde(rename = "burrow")]
    Burrow,

    #[serde(rename = "nestSite")]
    NestSite,

    #[serde(rename = "carcass")]
    Carcass,

    #[serde(rename = "waterSource")]
    WaterSource,

    #[serde(rename = "fruitingTree")]
    FruitingTree,
}

/// Media file recorded during a deployment.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Medium {
    /// Unique identifier of the media file.
    #[serde(rename = "mediaID")]
    pub media_id: String,

    /// Identifier of the deployment the media file belongs to. Foreign key to deployments.deploymentID.
    #[serde(rename = "deploymentID")]
    pub deployment_id: String,

    /// Method used to capture the media file.
    #[serde(rename = "captureMethod")]
    pub capture_method: Option<CaptureMethod>,

    /// Date and time at which the media file was recorded. Formatted as an ISO 8601 string with timezone designator (YYYY-MM-DDThh:mm:ssZ or YYYY-MM-DDThh:mm:ss±hh:mm).
    pub timestamp: chrono::DateTime<chrono::FixedOffset>,

    /// URL or relative path to the media file, respectively for externally hosted files or files that are part of the package.
    /// TODO: Ensure match ^(?=^[^./~])(^((?!\.{2}).)*$).*$
    #[serde(rename = "filePath")]
    pub file_path: String,

    /// false if the media file is not publicly accessible (e.g. to protect the privacy of people).
    #[serde(rename = "filePublic")]
    pub file_public: bool,

    /// Name of the media file. If provided, one should be able to sort media files chronologically within a deployment on timestamp (first) and fileName (second).
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,

    /// Mediatype of the media file. Expressed as an IANA Media Type.
    ///
    /// TODO: ^(image|video|audio)/.*$
    #[serde(rename = "fileMediatype")]
    pub file_mediatype: String,

    /// EXIF data of the media file. Formatted as a valid JSON object.
    #[serde(rename = "exifData")]
    pub exif_data: Option<serde_json::Value>,

    /// true if the media file is deemed of interest (e.g. an exemplar image of an individual).
    pub favorite: Option<bool>,

    /// Comments or notes about the media file.
    pub comments: Option<String>,
}

/// Method used to capture the media file.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CaptureMethod {
    #[serde(rename = "activityDetection")]
    ActivityDetection,

    #[serde(rename = "timeLapse")]
    TimeLapse,
}

/// An observation derived from a media file.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Observation {
    /// Unique identifier of the observation.
    #[serde(rename = "observationID")]
    pub observation_id: String,

    /// Identifier of the deployment the observation belongs to. Foreign key to deployments.deploymentID.
    #[serde(rename = "deploymentID")]
    pub deployment_id: String,

    /// Identifier of the media file that was classified. Only applicable for media-based observations (observationLevel = media). Foreign key to media.mediaID.
    #[serde(rename = "mediaID")]
    pub media_id: Option<String>,

    /// Identifier of the event the observation belongs to. Facilitates linking event-based and media-based observations with a permanent identifier.
    #[serde(rename = "eventID")]
    pub event_id: Option<String>,

    /// Date and time at which the event started. Formatted as an ISO 8601 string with timezone designator (YYYY-MM-DDThh:mm:ssZ or YYYY-MM-DDThh:mm:ss±hh:mm).
    #[serde(rename = "eventStart")]
    pub event_start: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// Date and time at which the event ended. Formatted as an ISO 8601 string with timezone designator (YYYY-MM-DDThh:mm:ssZ or YYYY-MM-DDThh:mm:ss±hh:mm).
    #[serde(rename = "eventEnd")]
    pub event_end: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// Level at which the observation was classified. media for media-based observations that are directly associated with a media file (mediaID). These are especially useful for machine learning and don’t need to be mutually exclusive (e.g. multiple classifications are allowed). event for event-based observations that consider an event (comprising a collection of media files). These are especially useful for ecological research and should be mutually exclusive, so that their count can be summed.
    #[serde(rename = "observationLevel")]
    pub observation_level: ObservationLevel,

    /// Type of the observation. All categories in this vocabulary have to be understandable from an AI point of view. unknown describes classifications with a classificationProbability below some predefined threshold i.e. neither humans nor AI can say what was recorded.
    #[serde(rename = "observationType")]
    pub observation_type: ObservationType,

    /// Type of the camera setup action (if any) associated with the observation.
    #[serde(rename = "cameraSetupType")]
    pub camera_setup_type: Option<CameraSetupType>,

    /// Scientific name of the observed individual(s).
    #[serde(rename = "scientificName")]
    pub scientific_name: Option<String>,

    /// Number of observed individuals (optionally of given life stage, sex and behavior).
    pub count: Option<u32>,

    /// Age class or life stage of the observed individual(s).
    #[serde(rename = "lifeStage")]
    pub life_stage: Option<LifeStage>,

    /// Sex of the observed individual(s)
    pub sex: Option<Sex>,

    /// Dominant behavior of the observed individual(s), preferably expressed as controlled values (e.g. grazing, browsing, rooting, vigilance, running, walking). Formatted as a pipe (|) separated list for multiple values, with the dominant behavior listed first.
    pub behavior: Option<String>,

    /// Identifier of the observed individual.
    #[serde(rename = "individualID")]
    pub individual_id: Option<String>,

    /// Distance from the camera to the observed individual identified by individualID. Expressed in meters. Required for distance analyses (e.g. Howe et al. 2017) and random encounter modelling (e.g. Rowcliffe et al. 2011).
    #[serde(rename = "individualPositionRadius")]
    pub individual_position_radius: Option<f64>,

    /// Angular distance from the camera view centerline to the observed individual identified by individualID. Expressed in degrees, with negative values left, 0 straight ahead and positive values right. Required for distance analyses (e.g. Howe et al. 2017) and random encounter modelling (e.g. Rowcliffe et al. 2011).
    #[serde(rename = "individualPositionAngle")]
    pub individual_position_angle: Option<f64>,

    /// Average movement speed of the observed individual identified by individualID. Expressed in meters per second. Required for random encounter modelling (e.g. Rowcliffe et al. 2016).
    #[serde(rename = "individualSpeed")]
    pub individual_speed: Option<f64>,

    /// Horizontal position of the top-left corner of a bounding box that encompasses the observed individual(s) in the media file identified by mediaID. Or the horizontal position of an object in that media file. Measured from the left and relative to media file width.
    #[serde(rename = "bboxX")]
    pub bbox_x: Option<f64>,

    /// Vertical position of the top-left corner of a bounding box that encompasses the observed individual(s) in the media file identified by mediaID. Or the vertical position of an object in that media file. Measured from the top and relative to the media file height.
    #[serde(rename = "bboxY")]
    pub bbox_y: Option<f64>,

    /// Width of a bounding box that encompasses the observed individual(s) in the media file identified by mediaID. Measured from the left of the bounding box and relative to the media file width.
    #[serde(rename = "bboxWidth")]
    pub bbox_width: Option<f64>,

    /// Height of the bounding box that encompasses the observed individual(s) in the media file identified by mediaID. Measured from the top of the bounding box and relative to the media file height.
    #[serde(rename = "bboxHeight")]
    pub bbox_height: Option<f64>,

    /// Method (most recently) used to classify the observation.
    #[serde(rename = "classificationMethod")]
    pub classification_method: Option<ClassificationMethod>,

    /// Name or identifier of the person or AI algorithm that (most recently) classified the observation.
    #[serde(rename = "classifiedBy")]
    pub classified_by: Option<String>,

    /// Date and time of the (most recent) classification. Formatted as an ISO 8601 string with timezone designator (YYYY-MM-DDThh:mm:ssZ or YYYY-MM-DDThh:mm:ss±hh:mm).
    #[serde(rename = "classificationTimestamp")]
    pub classification_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// Degree of certainty of the (most recent) classification. Expressed as a probability, with 1 being maximum certainty. Omit or provide an approximate probability for human classifications.
    #[serde(rename = "classificationProbability")]
    pub classification_probability: Option<f64>,

    /// Tag(s) associated with the observation. Formatted as a pipe (|) separated list for multiple values, with values optionally formatted as key:value pairs.
    #[serde(rename = "observationTags")]
    pub observation_tags: Option<String>,

    /// Comments or notes about the observation.
    #[serde(rename = "observationComments")]
    pub observation_comments: Option<String>,
}

/// Level at which the observation was classified.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ObservationLevel {
    #[serde(rename = "media")]
    Media,

    #[serde(rename = "event")]
    Event,
}

/// Type of the observation.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ObservationType {
    #[serde(rename = "animal")]
    Animal,

    #[serde(rename = "human")]
    Human,

    #[serde(rename = "vehicle")]
    Vehicle,

    #[serde(rename = "blank")]
    Blank,

    #[serde(rename = "unknown")]
    Unknown,

    #[serde(rename = "unclassified")]
    Unclassified,
}

/// Type of the camera setup action associated with the observation.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CameraSetupType {
    #[serde(rename = "setup")]
    Setup,

    #[serde(rename = "calibration")]
    Calibration,
}

/// Life stage of the observed individual(s).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum LifeStage {
    #[serde(rename = "adult")]
    Adult,

    #[serde(rename = "subadult")]
    Subadult,

    #[serde(rename = "juvenile")]
    Juvenile,
}

/// Sex of the observed individual(s)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Sex {
    #[serde(rename = "female")]
    Female,

    #[serde(rename = "male")]
    Male,
}

/// Classification method used to classify the observation.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ClassificationMethod {
    #[serde(rename = "human")]
    Human,

    #[serde(rename = "machine")]
    Machine,
}

/// Trait for reading and writing data tables in CSV format.
pub trait ReadDataPackageCsv<T: Serialize + for<'de> Deserialize<'de>> {
    /// Read data from a CSV file.
    fn from_file(path: &str) -> Result<Vec<T>, csv::Error>
    where
        T: Sized,
    {
        let mut rdr = csv::Reader::from_path(path)?;
        let mut data = Vec::new();
        for result in rdr.deserialize() {
            let record: T = result?;
            data.push(record);
        }
        Ok(data)
    }

    /// Read data from a CSV file at a URL.
    fn from_url(url: &str) -> Result<Vec<T>, FromUrlError>
    where
        T: Sized,
    {
        let bytes = reqwest::blocking::get(url)
            .map_err(FromUrlError::Reqwest)?
            .bytes()
            .map_err(FromUrlError::Reqwest)?;

        let mut rdr = csv::Reader::from_reader(bytes.reader());

        let mut data = Vec::new();
        for result in rdr.deserialize() {
            let record: T = result.map_err(FromUrlError::Csv)?;
            data.push(record);
        }
        Ok(data)
    }
}

impl ReadDataPackageCsv<Deployment> for Deployment {}
impl ReadDataPackageCsv<Medium> for Medium {}
impl ReadDataPackageCsv<Observation> for Observation {}

pub trait WriteDataPackageCsv<T: Serialize + for<'de> Deserialize<'de>>
where
    Self: IntoIterator<Item = T> + Clone,
{
    /// Write data to a CSV file.
    fn to_file(&self, path: &str) -> Result<(), csv::Error> {
        let mut wtr = csv::Writer::from_path(path)?;
        for record in self.clone().into_iter() {
            wtr.serialize(record)?;
        }
        wtr.flush()?;
        Ok(())
    }
}

impl WriteDataPackageCsv<Deployment> for Vec<Deployment> {}
impl WriteDataPackageCsv<Medium> for Vec<Medium> {}
impl WriteDataPackageCsv<Observation> for Vec<Observation> {}

/// Error type for `from_url`.
#[derive(Debug)]
pub enum FromUrlError {
    Reqwest(reqwest::Error),
    Csv(csv::Error),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deployment_from_file() {
        let deployments = Deployment::from_file("fixtures/deployments.csv").unwrap();
        assert_eq!(deployments.len(), 4);
    }

    #[test]
    fn deployment_from_url() {
        let deployments = Deployment::from_url(
            "https://github.com/tdwg/camtrap-dp/raw/1.0/example/deployments.csv",
        )
        .unwrap();
        assert_eq!(deployments.len(), 4);
    }

    #[test]
    fn medium_from_file() {
        let data = Medium::from_file("fixtures/media.csv").unwrap();
        assert_eq!(data.len(), 423);
    }

    #[test]
    fn medium_from_url() {
        let data = Medium::from_url("https://github.com/tdwg/camtrap-dp/raw/1.0/example/media.csv")
            .unwrap();

        assert_eq!(data.len(), 423);
    }

    #[test]
    fn observation_from_file() {
        let data = Observation::from_file("fixtures/observations.csv").unwrap();
        assert_eq!(data.len(), 549);
    }

    #[test]
    fn observation_from_url() {
        let data = Observation::from_url(
            "https://github.com/tdwg/camtrap-dp/raw/1.0/example/observations.csv",
        )
        .unwrap();

        assert_eq!(data.len(), 549);
    }

    #[test]
    fn deployment_to_file() {
        let deployments = Deployment::from_file("fixtures/deployments.csv").unwrap();
        deployments.to_file("fixtures/deployments_out.csv").unwrap();
        let deployments_out = Deployment::from_file("fixtures/deployments_out.csv").unwrap();
        assert_eq!(deployments, deployments_out);
    }

    #[test]
    fn medium_to_file() {
        let data = Medium::from_file("fixtures/media.csv").unwrap();
        data.to_file("fixtures/media_out.csv").unwrap();
        let data_out = Medium::from_file("fixtures/media_out.csv").unwrap();
        assert_eq!(data, data_out);
    }

    #[test]
    fn observation_to_file() {
        let data = Observation::from_file("fixtures/observations.csv").unwrap();
        data.to_file("fixtures/observations_out.csv").unwrap();
        let data_out = Observation::from_file("fixtures/observations_out.csv").unwrap();
        assert_eq!(data, data_out);
    }
}
