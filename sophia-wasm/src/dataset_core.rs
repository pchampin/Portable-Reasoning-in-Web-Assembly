//! This file implements the DatasetCore interface from the RDF.JS
//! specification. The implementation also offers many services from the
//! Dataset class.

//#![deny(missing_docs)]

use crate::wasm_bindgen_dataset;
use crate::wasm_bindgen_wrappeddataset;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use sophia::dataset::inmem::FastDataset;
use sophia::dataset::inmem::LightDataset;
use sophia::term::BoxTerm;

use bjdatasets::treeddataset::TreedDataset;
use bjdatasets::fulldataset::FullIndexDataset;
use crate::btreeddataset_anti::BTreedDatasetAntiWrapper;
use crate::dataset_into_vector_wrapper::VecOrDatasetWrapper;



// Dataset structure created by the factory
wasm_bindgen_dataset!(TreedDataset, "TreedDataset", SophiaExportDataset);

// Other usable datasets
wasm_bindgen_dataset!(FastDataset, "FastDataset");
wasm_bindgen_dataset!(LightDataset, "LightDataset");
wasm_bindgen_dataset!(FullIndexDataset, "FullDataset");

// Array Dataset (which is not a real set)
type ArryDataset = Vec<([BoxTerm; 3], Option<BoxTerm>)>;
wasm_bindgen_dataset!(ArryDataset, "ArrayDataset");


// A dataset that redefines the match method
wasm_bindgen_wrappeddataset!(BTreedDatasetAntiWrapper, "AntiTreedDataset");

// Datasets that fills an array instead of the base complicated structure
type TreedDatasetIntoArrayWrapper = VecOrDatasetWrapper<TreedDataset>;
type FastDatasetIntoArrayWrapper = VecOrDatasetWrapper<FastDataset>;
type LightDatasetIntoArrayWrapper = VecOrDatasetWrapper<LightDataset>;
type FullIndexDatasetIntoArrayWrapper = VecOrDatasetWrapper<FullIndexDataset>;

wasm_bindgen_wrappeddataset!(TreedDatasetIntoArrayWrapper, "TreedDatasetToA");
wasm_bindgen_wrappeddataset!(FastDatasetIntoArrayWrapper, "FastDatasetToA");
wasm_bindgen_wrappeddataset!(LightDatasetIntoArrayWrapper, "LightDatasetToA");
wasm_bindgen_wrappeddataset!(FullIndexDatasetIntoArrayWrapper, "FullDatasetToA");
