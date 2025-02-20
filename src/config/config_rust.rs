// Configuration file for Rust
pub const VERBOS_DISABLE: &str = 0;
pub const VERBOS_L1: &str = 1;
pub const VERBOS_L2: &str = 2;
pub const VERBOS: &str = VERBOS_L1;
pub const WINDOWS: &str = 0;
pub const LINUX: &str = 1;
pub const MAC: &str = 2;
pub const OS: &str = WINDOWS;
pub const FPGA: &str = 0;
pub const MICROCONTROLER: &str = 1;
pub const SOFT_PROCESSOR: &str = 2;
pub const DEVICE_TYPE: &str = MICROCONTROLER;
pub const description: &str = "Class Vector Modes and Configurations";
pub const TARGET_DEVICE: &str = "xczu9eg-ffvb1156-2-e";
pub const AMD_ZYBO: &str = 0;
pub const AMD_ZEDBOARD: &str = 1;
pub const AMD_ZCU106: &str = 2;
pub const AMD_ZCU102: &str = 3;
pub const AMD_VCU108: &str = 4;
pub const TARGET_BOARD: &str = AMD_ZCU102;
pub const OP_FREQ: &str = 200;
pub const USE_VIV_EN: &str = 1;
pub const VIV_OPT_DEFAULT: &str = 0;
pub const VIV_OPT_RESOURCE: &str = 1;
pub const VIV_OPT_SPEED: &str = 2;
pub const VIV_OPT_PERFORMANCE: &str = 3;
pub const VIV_OPT_METHOD: &str = VIV_OPT_DEFAULT;
pub const HLS: &str = 0;
pub const SYSTEM_VERILOG: &str = 1;
pub const VHDL: &str = 2;
pub const VERILOG: &str = 3;
pub const HDL_LANG: &str = HLS;
pub const PYTHON: &str = 4;
pub const MATLAB: &str = 5;
pub const CPP: &str = 6;
pub const C: &str = 7;
pub const RUST: &str = 8;
pub const MODEL_LANG: &str = PYTHON;
pub const DS_NAME: &str = "CARDIO";
pub const DS_SIZE: &str = 2126;
pub const DS_TRAIN_SIZE: &str = 1594;
pub const DS_TEST_SIZE: &str = 532;
pub const DS_VALIDATION_SIZE: &str = 500;
pub const DS_FEATURE_SIZE: &str = 21;
pub const DS_DATA_TYPE: &str = "FP";
pub const DS_DATA_RANGE_MIN: &str = 0;
pub const DS_DATA_RANGE_MAX: &str = 1;
pub const DS_CLASSES_SIZE: &str = 3;
pub const TRAIN_ON_HW: &str = 0;
pub const RETRAIN_ON_HW: &str = 0;
pub const EPOCH: &str = 1;
pub const HD_DIM: &str = 512;
pub const BINARY: &str = 0;
pub const BIPOLAR: &str = 1;
pub const HD_DATA_TYPE: &str = BINARY;
pub const DENSE: &str = 0;
pub const SPARSE: &str = 1;
pub const HD_MODE: &str = DENSE;
pub const SIMI_COS: &str = 0;
pub const SIMI_DPROD: &str = 1;
pub const SIMI_HAM: &str = 2;
pub const HD_SIMI_METHOD: &str = SIMI_HAM;
pub const APPROX_SQRT_ON_HW: &str = 0;
pub const APPROX_SQRT_ON_MODEL: &str = 0;
pub const HD_SIMI_W_BITS: &str = 32;
pub const LINEAR: &str = 0;
pub const APPROX: &str = 1;
pub const THERMOMETER: &str = 2;
pub const HD_LV_TYPE: &str = LINEAR;
pub const HD_LV_LEN: &str = 32;
pub const SPARSITY_FACTOR_X10: &str = 5;
pub const PROBLEM_TYPE_CLASSIFICATION: &str = 0;
pub const PROBLEM_TYPE_CLUSTERING: &str = 1;
pub const PROBLEM_TYPE_REGRESSION: &str = 2;
pub const PROBLEM_TYPE: &str = PROBLEM_TYPE_CLASSIFICATION;
pub const DI_M_STREAM: &str = 0;
pub const DI_M_PARTIAL_PARALLEL: &str = 1;
pub const DI_M_PARALLEL: &str = 2;
pub const IN_DATA_MODE: &str = DI_M_STREAM;
pub const AXI_CNTR_PORT_EN: &str = 0;
pub const PARALLELISM_FEATURES: &str = 1;
pub const PARALLELISM_CLASS: &str = 1;
pub const DI_PARALLEL_W_BITS: &str = 64;
pub const FRAME_SIZE_MIN: &str = 64;
pub const DI_QUANT_BITS: &str = 6;
pub const HD_DATA_W_BITS: &str = 1;
pub const BV_M_INT_MEM: &str = 0;
pub const BV_M_EXT: &str = 1;
pub const BV_M_PERMUTATION: &str = 2;
pub const BV_M_RND_GEN: &str = 3;
pub const BV_MODE: &str = BV_M_INT_MEM;
pub const BV_DATA_W_BITS: &str = 1;
pub const BV_IN_DATA_W_BITS: &str = 64;
pub const BV_RND_GEN_W_BITS: &str = 64;
pub const LV_M_INT_MEM: &str = 0;
pub const LV_M_EXT: &str = 1;
pub const LV_M_LOGIC: &str = 2;
pub const LV_M_HDL_GEN: &str = 3;
pub const LV_MODE: &str = LV_M_INT_MEM;
pub const LV_DATA_W_BITS: &str = 1;
pub const LV_IN_DATA_W_BITS: &str = 64;
pub const LV_M_APPROX_RND_GEN_W_BITS: &str = 64;
pub const CV_M_INT_MEM: &str = 0;
pub const CV_M_EXT: &str = 1;
pub const CV_M_HDL_GEN: &str = 2;
pub const CV_MODE: &str = CV_M_INT_MEM;
pub const CV_DATA_W_BITS: &str = 1;
pub const CV_IN_DATA_W_BITS: &str = 64;
pub const DI_FEATUREID_W_BITS: &str = 10;
pub const DI_FRAMEID_W_BITS: &str = 10;
pub const DO_CLASS_W_BITS: &str = 6;
pub const DO_STATUS_W_BITS: &str = 5;
pub const HD_BUNDLE_W_BITS: &str = 32;
pub const ENCODING_RECORD: &str = 0;
pub const ENCODING_NGRAM: &str = 1;
pub const ENCODING_TECHNIQUE: &str = ENCODING_RECORD;
pub const N_GRAM_SIZE: &str = 1;
pub const N_GRAM: &str = 0;
pub const CLIPPING_DISABLE: &str = 0;
pub const CLIPPING_BINARY: &str = 1;
pub const CLIPPING_TERNARY: &str = 2;
pub const CLIPPING_QUANTIZED: &str = 3;
pub const CLIPPING_POWERTWO: &str = 4;
pub const CLIPPING_QUANTIZED_POWERTWO: &str = 5;
pub const CLIPPING_THRESHOLD: &str = 6;
pub const CLIPPING_ENCODING: &str = CLIPPING_BINARY;
pub const CLIPPING_CLASS: &str = CLIPPING_BINARY;
pub const CLIPPING_BINARY_FOR_VALUE_EQ_HALF_HD_DIM_USE_TOGGLING: &str = 0;
pub const CLIPPING_BINARY_FOR_VALUE_EQ_HALF_HD_DIM_SET_ZERO: &str = 1;
pub const CLIPPING_BINARY_METHOD_FOR_VALUE_EQ_HALF_HD_DIM: &str = CLIPPING_BINARY_FOR_VALUE_EQ_HALF_HD_DIM_SET_ZERO;
pub const HW_CLIPPING_AFTER_TRAIN: &str = 0;
pub const QUANT_MIN: &str = 0;
pub const QUANT_MAX: &str = 1;
pub const LR_CONSTANT: &str = 0;
pub const LR_ITER: &str = 1;
pub const LR_DATA: &str = 2;
pub const LR_HYBRID: &str = 3;
pub const LR_DECAY: &str = LR_CONSTANT;
pub const MAX_LEARNING_RATE: &str = 10;
pub const BETA_LR: &str = 3;
pub const RETRAIN: &str = 0;
pub const VITIS_HLS_XILINX_PATH: &str = "C:/Xilinx/Vitis_HLS/2023.1/bin/vitis_hls.bat";
pub const VIVADO_XILINX_PATH: &str = "C:/Xilinx/Vivado/2023.1/bin/vivado.bat";
pub const VITIS_XILINX_PATH: &str = "C:/Xilinx/Vitis/2023.1/bin/vitis.bat";
