/* automatically generated by rust-bindgen */

use libc::size_t;
use core::{CvArr, CvBox2D, CvChain, CvContour, CvHistogram, CvMat, CvMemStorage, CvPoint2D32f, CvPoint, CvRect, CvScalar, CvSeqBlock, CvSeq, CvSetElem, CvSet, CvSize, CvSlice, CvTermCriteria, IplConvKernel, Struct_CvSeq, schar};

#[repr(C)]
#[derive(Copy)]
pub struct Struct_CvConnectedComp {
    pub area: ::libc::c_double,
    pub value: CvScalar,
    pub rect: CvRect,
    pub contour: *mut CvSeq,
}
impl ::std::default::Default for Struct_CvConnectedComp {
    fn default() -> Struct_CvConnectedComp { unsafe { ::std::mem::zeroed() } }
}
pub type CvConnectedComp = Struct_CvConnectedComp;
pub type Enum_Unnamed1 = ::libc::c_uint;
pub const CV_BLUR_NO_SCALE: ::libc::c_uint = 0;
pub const CV_BLUR: ::libc::c_uint = 1;
pub const CV_GAUSSIAN: ::libc::c_uint = 2;
pub const CV_MEDIAN: ::libc::c_uint = 3;
pub const CV_BILATERAL: ::libc::c_uint = 4;
pub type Enum_Unnamed2 = ::libc::c_uint;
pub const CV_GAUSSIAN_5x5: ::libc::c_uint = 7;
pub type Enum_Unnamed3 = ::libc::c_int;
pub const CV_SCHARR: ::libc::c_int = -1;
pub const CV_MAX_SOBEL_KSIZE: ::libc::c_int = 7;
pub type Enum_Unnamed4 = ::libc::c_uint;
pub const CV_BGR2BGRA: ::libc::c_uint = 0;
pub const CV_RGB2RGBA: ::libc::c_uint = 0;
pub const CV_BGRA2BGR: ::libc::c_uint = 1;
pub const CV_RGBA2RGB: ::libc::c_uint = 1;
pub const CV_BGR2RGBA: ::libc::c_uint = 2;
pub const CV_RGB2BGRA: ::libc::c_uint = 2;
pub const CV_RGBA2BGR: ::libc::c_uint = 3;
pub const CV_BGRA2RGB: ::libc::c_uint = 3;
pub const CV_BGR2RGB: ::libc::c_uint = 4;
pub const CV_RGB2BGR: ::libc::c_uint = 4;
pub const CV_BGRA2RGBA: ::libc::c_uint = 5;
pub const CV_RGBA2BGRA: ::libc::c_uint = 5;
pub const CV_BGR2GRAY: ::libc::c_uint = 6;
pub const CV_RGB2GRAY: ::libc::c_uint = 7;
pub const CV_GRAY2BGR: ::libc::c_uint = 8;
pub const CV_GRAY2RGB: ::libc::c_uint = 8;
pub const CV_GRAY2BGRA: ::libc::c_uint = 9;
pub const CV_GRAY2RGBA: ::libc::c_uint = 9;
pub const CV_BGRA2GRAY: ::libc::c_uint = 10;
pub const CV_RGBA2GRAY: ::libc::c_uint = 11;
pub const CV_BGR2BGR565: ::libc::c_uint = 12;
pub const CV_RGB2BGR565: ::libc::c_uint = 13;
pub const CV_BGR5652BGR: ::libc::c_uint = 14;
pub const CV_BGR5652RGB: ::libc::c_uint = 15;
pub const CV_BGRA2BGR565: ::libc::c_uint = 16;
pub const CV_RGBA2BGR565: ::libc::c_uint = 17;
pub const CV_BGR5652BGRA: ::libc::c_uint = 18;
pub const CV_BGR5652RGBA: ::libc::c_uint = 19;
pub const CV_GRAY2BGR565: ::libc::c_uint = 20;
pub const CV_BGR5652GRAY: ::libc::c_uint = 21;
pub const CV_BGR2BGR555: ::libc::c_uint = 22;
pub const CV_RGB2BGR555: ::libc::c_uint = 23;
pub const CV_BGR5552BGR: ::libc::c_uint = 24;
pub const CV_BGR5552RGB: ::libc::c_uint = 25;
pub const CV_BGRA2BGR555: ::libc::c_uint = 26;
pub const CV_RGBA2BGR555: ::libc::c_uint = 27;
pub const CV_BGR5552BGRA: ::libc::c_uint = 28;
pub const CV_BGR5552RGBA: ::libc::c_uint = 29;
pub const CV_GRAY2BGR555: ::libc::c_uint = 30;
pub const CV_BGR5552GRAY: ::libc::c_uint = 31;
pub const CV_BGR2XYZ: ::libc::c_uint = 32;
pub const CV_RGB2XYZ: ::libc::c_uint = 33;
pub const CV_XYZ2BGR: ::libc::c_uint = 34;
pub const CV_XYZ2RGB: ::libc::c_uint = 35;
pub const CV_BGR2YCrCb: ::libc::c_uint = 36;
pub const CV_RGB2YCrCb: ::libc::c_uint = 37;
pub const CV_YCrCb2BGR: ::libc::c_uint = 38;
pub const CV_YCrCb2RGB: ::libc::c_uint = 39;
pub const CV_BGR2HSV: ::libc::c_uint = 40;
pub const CV_RGB2HSV: ::libc::c_uint = 41;
pub const CV_BGR2Lab: ::libc::c_uint = 44;
pub const CV_RGB2Lab: ::libc::c_uint = 45;
pub const CV_BayerBG2BGR: ::libc::c_uint = 46;
pub const CV_BayerGB2BGR: ::libc::c_uint = 47;
pub const CV_BayerRG2BGR: ::libc::c_uint = 48;
pub const CV_BayerGR2BGR: ::libc::c_uint = 49;
pub const CV_BayerBG2RGB: ::libc::c_uint = 48;
pub const CV_BayerGB2RGB: ::libc::c_uint = 49;
pub const CV_BayerRG2RGB: ::libc::c_uint = 46;
pub const CV_BayerGR2RGB: ::libc::c_uint = 47;
pub const CV_BGR2Luv: ::libc::c_uint = 50;
pub const CV_RGB2Luv: ::libc::c_uint = 51;
pub const CV_BGR2HLS: ::libc::c_uint = 52;
pub const CV_RGB2HLS: ::libc::c_uint = 53;
pub const CV_HSV2BGR: ::libc::c_uint = 54;
pub const CV_HSV2RGB: ::libc::c_uint = 55;
pub const CV_Lab2BGR: ::libc::c_uint = 56;
pub const CV_Lab2RGB: ::libc::c_uint = 57;
pub const CV_Luv2BGR: ::libc::c_uint = 58;
pub const CV_Luv2RGB: ::libc::c_uint = 59;
pub const CV_HLS2BGR: ::libc::c_uint = 60;
pub const CV_HLS2RGB: ::libc::c_uint = 61;
pub const CV_BayerBG2BGR_VNG: ::libc::c_uint = 62;
pub const CV_BayerGB2BGR_VNG: ::libc::c_uint = 63;
pub const CV_BayerRG2BGR_VNG: ::libc::c_uint = 64;
pub const CV_BayerGR2BGR_VNG: ::libc::c_uint = 65;
pub const CV_BayerBG2RGB_VNG: ::libc::c_uint = 64;
pub const CV_BayerGB2RGB_VNG: ::libc::c_uint = 65;
pub const CV_BayerRG2RGB_VNG: ::libc::c_uint = 62;
pub const CV_BayerGR2RGB_VNG: ::libc::c_uint = 63;
pub const CV_BGR2HSV_FULL: ::libc::c_uint = 66;
pub const CV_RGB2HSV_FULL: ::libc::c_uint = 67;
pub const CV_BGR2HLS_FULL: ::libc::c_uint = 68;
pub const CV_RGB2HLS_FULL: ::libc::c_uint = 69;
pub const CV_HSV2BGR_FULL: ::libc::c_uint = 70;
pub const CV_HSV2RGB_FULL: ::libc::c_uint = 71;
pub const CV_HLS2BGR_FULL: ::libc::c_uint = 72;
pub const CV_HLS2RGB_FULL: ::libc::c_uint = 73;
pub const CV_LBGR2Lab: ::libc::c_uint = 74;
pub const CV_LRGB2Lab: ::libc::c_uint = 75;
pub const CV_LBGR2Luv: ::libc::c_uint = 76;
pub const CV_LRGB2Luv: ::libc::c_uint = 77;
pub const CV_Lab2LBGR: ::libc::c_uint = 78;
pub const CV_Lab2LRGB: ::libc::c_uint = 79;
pub const CV_Luv2LBGR: ::libc::c_uint = 80;
pub const CV_Luv2LRGB: ::libc::c_uint = 81;
pub const CV_BGR2YUV: ::libc::c_uint = 82;
pub const CV_RGB2YUV: ::libc::c_uint = 83;
pub const CV_YUV2BGR: ::libc::c_uint = 84;
pub const CV_YUV2RGB: ::libc::c_uint = 85;
pub const CV_BayerBG2GRAY: ::libc::c_uint = 86;
pub const CV_BayerGB2GRAY: ::libc::c_uint = 87;
pub const CV_BayerRG2GRAY: ::libc::c_uint = 88;
pub const CV_BayerGR2GRAY: ::libc::c_uint = 89;
pub const CV_YUV2RGB_NV12: ::libc::c_uint = 90;
pub const CV_YUV2BGR_NV12: ::libc::c_uint = 91;
pub const CV_YUV2RGB_NV21: ::libc::c_uint = 92;
pub const CV_YUV2BGR_NV21: ::libc::c_uint = 93;
pub const CV_YUV420sp2RGB: ::libc::c_uint = 92;
pub const CV_YUV420sp2BGR: ::libc::c_uint = 93;
pub const CV_YUV2RGBA_NV12: ::libc::c_uint = 94;
pub const CV_YUV2BGRA_NV12: ::libc::c_uint = 95;
pub const CV_YUV2RGBA_NV21: ::libc::c_uint = 96;
pub const CV_YUV2BGRA_NV21: ::libc::c_uint = 97;
pub const CV_YUV420sp2RGBA: ::libc::c_uint = 96;
pub const CV_YUV420sp2BGRA: ::libc::c_uint = 97;
pub const CV_YUV2RGB_YV12: ::libc::c_uint = 98;
pub const CV_YUV2BGR_YV12: ::libc::c_uint = 99;
pub const CV_YUV2RGB_IYUV: ::libc::c_uint = 100;
pub const CV_YUV2BGR_IYUV: ::libc::c_uint = 101;
pub const CV_YUV2RGB_I420: ::libc::c_uint = 100;
pub const CV_YUV2BGR_I420: ::libc::c_uint = 101;
pub const CV_YUV420p2RGB: ::libc::c_uint = 98;
pub const CV_YUV420p2BGR: ::libc::c_uint = 99;
pub const CV_YUV2RGBA_YV12: ::libc::c_uint = 102;
pub const CV_YUV2BGRA_YV12: ::libc::c_uint = 103;
pub const CV_YUV2RGBA_IYUV: ::libc::c_uint = 104;
pub const CV_YUV2BGRA_IYUV: ::libc::c_uint = 105;
pub const CV_YUV2RGBA_I420: ::libc::c_uint = 104;
pub const CV_YUV2BGRA_I420: ::libc::c_uint = 105;
pub const CV_YUV420p2RGBA: ::libc::c_uint = 102;
pub const CV_YUV420p2BGRA: ::libc::c_uint = 103;
pub const CV_YUV2GRAY_420: ::libc::c_uint = 106;
pub const CV_YUV2GRAY_NV21: ::libc::c_uint = 106;
pub const CV_YUV2GRAY_NV12: ::libc::c_uint = 106;
pub const CV_YUV2GRAY_YV12: ::libc::c_uint = 106;
pub const CV_YUV2GRAY_IYUV: ::libc::c_uint = 106;
pub const CV_YUV2GRAY_I420: ::libc::c_uint = 106;
pub const CV_YUV420sp2GRAY: ::libc::c_uint = 106;
pub const CV_YUV420p2GRAY: ::libc::c_uint = 106;
pub const CV_YUV2RGB_UYVY: ::libc::c_uint = 107;
pub const CV_YUV2BGR_UYVY: ::libc::c_uint = 108;
pub const CV_YUV2RGB_Y422: ::libc::c_uint = 107;
pub const CV_YUV2BGR_Y422: ::libc::c_uint = 108;
pub const CV_YUV2RGB_UYNV: ::libc::c_uint = 107;
pub const CV_YUV2BGR_UYNV: ::libc::c_uint = 108;
pub const CV_YUV2RGBA_UYVY: ::libc::c_uint = 111;
pub const CV_YUV2BGRA_UYVY: ::libc::c_uint = 112;
pub const CV_YUV2RGBA_Y422: ::libc::c_uint = 111;
pub const CV_YUV2BGRA_Y422: ::libc::c_uint = 112;
pub const CV_YUV2RGBA_UYNV: ::libc::c_uint = 111;
pub const CV_YUV2BGRA_UYNV: ::libc::c_uint = 112;
pub const CV_YUV2RGB_YUY2: ::libc::c_uint = 115;
pub const CV_YUV2BGR_YUY2: ::libc::c_uint = 116;
pub const CV_YUV2RGB_YVYU: ::libc::c_uint = 117;
pub const CV_YUV2BGR_YVYU: ::libc::c_uint = 118;
pub const CV_YUV2RGB_YUYV: ::libc::c_uint = 115;
pub const CV_YUV2BGR_YUYV: ::libc::c_uint = 116;
pub const CV_YUV2RGB_YUNV: ::libc::c_uint = 115;
pub const CV_YUV2BGR_YUNV: ::libc::c_uint = 116;
pub const CV_YUV2RGBA_YUY2: ::libc::c_uint = 119;
pub const CV_YUV2BGRA_YUY2: ::libc::c_uint = 120;
pub const CV_YUV2RGBA_YVYU: ::libc::c_uint = 121;
pub const CV_YUV2BGRA_YVYU: ::libc::c_uint = 122;
pub const CV_YUV2RGBA_YUYV: ::libc::c_uint = 119;
pub const CV_YUV2BGRA_YUYV: ::libc::c_uint = 120;
pub const CV_YUV2RGBA_YUNV: ::libc::c_uint = 119;
pub const CV_YUV2BGRA_YUNV: ::libc::c_uint = 120;
pub const CV_YUV2GRAY_UYVY: ::libc::c_uint = 123;
pub const CV_YUV2GRAY_YUY2: ::libc::c_uint = 124;
pub const CV_YUV2GRAY_Y422: ::libc::c_uint = 123;
pub const CV_YUV2GRAY_UYNV: ::libc::c_uint = 123;
pub const CV_YUV2GRAY_YVYU: ::libc::c_uint = 124;
pub const CV_YUV2GRAY_YUYV: ::libc::c_uint = 124;
pub const CV_YUV2GRAY_YUNV: ::libc::c_uint = 124;
pub const CV_RGBA2mRGBA: ::libc::c_uint = 125;
pub const CV_mRGBA2RGBA: ::libc::c_uint = 126;
pub const CV_RGB2YUV_I420: ::libc::c_uint = 127;
pub const CV_BGR2YUV_I420: ::libc::c_uint = 128;
pub const CV_RGB2YUV_IYUV: ::libc::c_uint = 127;
pub const CV_BGR2YUV_IYUV: ::libc::c_uint = 128;
pub const CV_RGBA2YUV_I420: ::libc::c_uint = 129;
pub const CV_BGRA2YUV_I420: ::libc::c_uint = 130;
pub const CV_RGBA2YUV_IYUV: ::libc::c_uint = 129;
pub const CV_BGRA2YUV_IYUV: ::libc::c_uint = 130;
pub const CV_RGB2YUV_YV12: ::libc::c_uint = 131;
pub const CV_BGR2YUV_YV12: ::libc::c_uint = 132;
pub const CV_RGBA2YUV_YV12: ::libc::c_uint = 133;
pub const CV_BGRA2YUV_YV12: ::libc::c_uint = 134;
pub const CV_COLORCVT_MAX: ::libc::c_uint = 135;
pub type Enum_Unnamed5 = ::libc::c_uint;
pub const CV_INTER_NN: ::libc::c_uint = 0;
pub const CV_INTER_LINEAR: ::libc::c_uint = 1;
pub const CV_INTER_CUBIC: ::libc::c_uint = 2;
pub const CV_INTER_AREA: ::libc::c_uint = 3;
pub const CV_INTER_LANCZOS4: ::libc::c_uint = 4;
pub type Enum_Unnamed6 = ::libc::c_uint;
pub const CV_WARP_FILL_OUTLIERS: ::libc::c_uint = 8;
pub const CV_WARP_INVERSE_MAP: ::libc::c_uint = 16;
pub type Enum_Unnamed7 = ::libc::c_uint;
pub const CV_SHAPE_RECT: ::libc::c_uint = 0;
pub const CV_SHAPE_CROSS: ::libc::c_uint = 1;
pub const CV_SHAPE_ELLIPSE: ::libc::c_uint = 2;
pub const CV_SHAPE_CUSTOM: ::libc::c_uint = 100;
pub type Enum_Unnamed8 = ::libc::c_uint;
pub const CV_MOP_ERODE: ::libc::c_uint = 0;
pub const CV_MOP_DILATE: ::libc::c_uint = 1;
pub const CV_MOP_OPEN: ::libc::c_uint = 2;
pub const CV_MOP_CLOSE: ::libc::c_uint = 3;
pub const CV_MOP_GRADIENT: ::libc::c_uint = 4;
pub const CV_MOP_TOPHAT: ::libc::c_uint = 5;
pub const CV_MOP_BLACKHAT: ::libc::c_uint = 6;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_CvMoments {
    pub m00: ::libc::c_double,
    pub m10: ::libc::c_double,
    pub m01: ::libc::c_double,
    pub m20: ::libc::c_double,
    pub m11: ::libc::c_double,
    pub m02: ::libc::c_double,
    pub m30: ::libc::c_double,
    pub m21: ::libc::c_double,
    pub m12: ::libc::c_double,
    pub m03: ::libc::c_double,
    pub mu20: ::libc::c_double,
    pub mu11: ::libc::c_double,
    pub mu02: ::libc::c_double,
    pub mu30: ::libc::c_double,
    pub mu21: ::libc::c_double,
    pub mu12: ::libc::c_double,
    pub mu03: ::libc::c_double,
    pub inv_sqrt_m00: ::libc::c_double,
}
impl ::std::default::Default for Struct_CvMoments {
    fn default() -> Struct_CvMoments { unsafe { ::std::mem::zeroed() } }
}
pub type CvMoments = Struct_CvMoments;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_CvHuMoments {
    pub hu1: ::libc::c_double,
    pub hu2: ::libc::c_double,
    pub hu3: ::libc::c_double,
    pub hu4: ::libc::c_double,
    pub hu5: ::libc::c_double,
    pub hu6: ::libc::c_double,
    pub hu7: ::libc::c_double,
}
impl ::std::default::Default for Struct_CvHuMoments {
    fn default() -> Struct_CvHuMoments { unsafe { ::std::mem::zeroed() } }
}
pub type CvHuMoments = Struct_CvHuMoments;
pub type Enum_Unnamed9 = ::libc::c_uint;
pub const CV_TM_SQDIFF: ::libc::c_uint = 0;
pub const CV_TM_SQDIFF_NORMED: ::libc::c_uint = 1;
pub const CV_TM_CCORR: ::libc::c_uint = 2;
pub const CV_TM_CCORR_NORMED: ::libc::c_uint = 3;
pub const CV_TM_CCOEFF: ::libc::c_uint = 4;
pub const CV_TM_CCOEFF_NORMED: ::libc::c_uint = 5;
pub type CvDistanceFunction =
    ::std::option::Option<extern "C" fn
                              (a: *const ::libc::c_float,
                               b: *const ::libc::c_float,
                               user_param: *mut ::libc::c_void)
                              -> ::libc::c_float>;
pub type Enum_Unnamed10 = ::libc::c_uint;
pub const CV_RETR_EXTERNAL: ::libc::c_uint = 0;
pub const CV_RETR_LIST: ::libc::c_uint = 1;
pub const CV_RETR_CCOMP: ::libc::c_uint = 2;
pub const CV_RETR_TREE: ::libc::c_uint = 3;
pub const CV_RETR_FLOODFILL: ::libc::c_uint = 4;
pub type Enum_Unnamed11 = ::libc::c_uint;
pub const CV_CHAIN_CODE: ::libc::c_uint = 0;
pub const CV_CHAIN_APPROX_NONE: ::libc::c_uint = 1;
pub const CV_CHAIN_APPROX_SIMPLE: ::libc::c_uint = 2;
pub const CV_CHAIN_APPROX_TC89_L1: ::libc::c_uint = 3;
pub const CV_CHAIN_APPROX_TC89_KCOS: ::libc::c_uint = 4;
pub const CV_LINK_RUNS: ::libc::c_uint = 5;
pub enum Struct__CvContourScanner { }
pub type CvContourScanner = *mut Struct__CvContourScanner;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_CvChainPtReader {
    pub header_size: ::libc::c_int,
    pub seq: *mut CvSeq,
    pub block: *mut CvSeqBlock,
    pub ptr: *mut schar,
    pub block_min: *mut schar,
    pub block_max: *mut schar,
    pub delta_index: ::libc::c_int,
    pub prev_elem: *mut schar,
    pub code: ::libc::c_char,
    pub pt: CvPoint,
    pub deltas: [[schar; 2us]; 8us],
}
impl ::std::default::Default for Struct_CvChainPtReader {
    fn default() -> Struct_CvChainPtReader { unsafe { ::std::mem::zeroed() } }
}
pub type CvChainPtReader = Struct_CvChainPtReader;
pub type CvSubdiv2DEdge = size_t;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_CvQuadEdge2D {
    pub flags: ::libc::c_int,
    pub pt: [*mut Struct_CvSubdiv2DPoint; 4us],
    pub next: [CvSubdiv2DEdge; 4us],
}
impl ::std::default::Default for Struct_CvQuadEdge2D {
    fn default() -> Struct_CvQuadEdge2D { unsafe { ::std::mem::zeroed() } }
}
pub type CvQuadEdge2D = Struct_CvQuadEdge2D;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_CvSubdiv2DPoint {
    pub flags: ::libc::c_int,
    pub first: CvSubdiv2DEdge,
    pub pt: CvPoint2D32f,
    pub id: ::libc::c_int,
}
impl ::std::default::Default for Struct_CvSubdiv2DPoint {
    fn default() -> Struct_CvSubdiv2DPoint { unsafe { ::std::mem::zeroed() } }
}
pub type CvSubdiv2DPoint = Struct_CvSubdiv2DPoint;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_CvSubdiv2D {
    pub flags: ::libc::c_int,
    pub header_size: ::libc::c_int,
    pub h_prev: *mut Struct_CvSeq,
    pub h_next: *mut Struct_CvSeq,
    pub v_prev: *mut Struct_CvSeq,
    pub v_next: *mut Struct_CvSeq,
    pub total: ::libc::c_int,
    pub elem_size: ::libc::c_int,
    pub block_max: *mut schar,
    pub ptr: *mut schar,
    pub delta_elems: ::libc::c_int,
    pub storage: *mut CvMemStorage,
    pub free_blocks: *mut CvSeqBlock,
    pub first: *mut CvSeqBlock,
    pub free_elems: *mut CvSetElem,
    pub active_count: ::libc::c_int,
    pub edges: *mut CvSet,
    pub quad_edges: ::libc::c_int,
    pub is_geometry_valid: ::libc::c_int,
    pub recent_edge: CvSubdiv2DEdge,
    pub topleft: CvPoint2D32f,
    pub bottomright: CvPoint2D32f,
}
impl ::std::default::Default for Struct_CvSubdiv2D {
    fn default() -> Struct_CvSubdiv2D { unsafe { ::std::mem::zeroed() } }
}
pub type CvSubdiv2D = Struct_CvSubdiv2D;
pub type Enum_CvSubdiv2DPointLocation = ::libc::c_int;
pub const CV_PTLOC_ERROR: ::libc::c_int = -2;
pub const CV_PTLOC_OUTSIDE_RECT: ::libc::c_int = -1;
pub const CV_PTLOC_INSIDE: ::libc::c_int = 0;
pub const CV_PTLOC_VERTEX: ::libc::c_int = 1;
pub const CV_PTLOC_ON_EDGE: ::libc::c_int = 2;
pub type CvSubdiv2DPointLocation = Enum_CvSubdiv2DPointLocation;
pub type Enum_CvNextEdgeType = ::libc::c_uint;
pub const CV_NEXT_AROUND_ORG: ::libc::c_uint = 0;
pub const CV_NEXT_AROUND_DST: ::libc::c_uint = 34;
pub const CV_PREV_AROUND_ORG: ::libc::c_uint = 17;
pub const CV_PREV_AROUND_DST: ::libc::c_uint = 51;
pub const CV_NEXT_AROUND_LEFT: ::libc::c_uint = 19;
pub const CV_NEXT_AROUND_RIGHT: ::libc::c_uint = 49;
pub const CV_PREV_AROUND_LEFT: ::libc::c_uint = 32;
pub const CV_PREV_AROUND_RIGHT: ::libc::c_uint = 2;
pub type CvNextEdgeType = Enum_CvNextEdgeType;
pub type Enum_Unnamed12 = ::libc::c_uint;
pub const CV_POLY_APPROX_DP: ::libc::c_uint = 0;
pub type Enum_Unnamed13 = ::libc::c_uint;
pub const CV_CONTOURS_MATCH_I1: ::libc::c_uint = 1;
pub const CV_CONTOURS_MATCH_I2: ::libc::c_uint = 2;
pub const CV_CONTOURS_MATCH_I3: ::libc::c_uint = 3;
pub type Enum_Unnamed14 = ::libc::c_uint;
pub const CV_CLOCKWISE: ::libc::c_uint = 1;
pub const CV_COUNTER_CLOCKWISE: ::libc::c_uint = 2;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_CvConvexityDefect {
    pub start: *mut CvPoint,
    pub end: *mut CvPoint,
    pub depth_point: *mut CvPoint,
    pub depth: ::libc::c_float,
}
impl ::std::default::Default for Struct_CvConvexityDefect {
    fn default() -> Struct_CvConvexityDefect {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type CvConvexityDefect = Struct_CvConvexityDefect;
pub type Enum_Unnamed15 = ::libc::c_uint;
pub const CV_COMP_CORREL: ::libc::c_uint = 0;
pub const CV_COMP_CHISQR: ::libc::c_uint = 1;
pub const CV_COMP_INTERSECT: ::libc::c_uint = 2;
pub const CV_COMP_BHATTACHARYYA: ::libc::c_uint = 3;
pub const CV_COMP_HELLINGER: ::libc::c_uint = 3;
pub type Enum_Unnamed16 = ::libc::c_uint;
pub const CV_DIST_MASK_3: ::libc::c_uint = 3;
pub const CV_DIST_MASK_5: ::libc::c_uint = 5;
pub const CV_DIST_MASK_PRECISE: ::libc::c_uint = 0;
pub type Enum_Unnamed17 = ::libc::c_uint;
pub const CV_DIST_LABEL_CCOMP: ::libc::c_uint = 0;
pub const CV_DIST_LABEL_PIXEL: ::libc::c_uint = 1;
pub type Enum_Unnamed18 = ::libc::c_int;
pub const CV_DIST_USER: ::libc::c_int = -1;
pub const CV_DIST_L1: ::libc::c_int = 1;
pub const CV_DIST_L2: ::libc::c_int = 2;
pub const CV_DIST_C: ::libc::c_int = 3;
pub const CV_DIST_L12: ::libc::c_int = 4;
pub const CV_DIST_FAIR: ::libc::c_int = 5;
pub const CV_DIST_WELSCH: ::libc::c_int = 6;
pub const CV_DIST_HUBER: ::libc::c_int = 7;
pub type Enum_Unnamed19 = ::libc::c_uint;
pub const CV_THRESH_BINARY: ::libc::c_uint = 0;
pub const CV_THRESH_BINARY_INV: ::libc::c_uint = 1;
pub const CV_THRESH_TRUNC: ::libc::c_uint = 2;
pub const CV_THRESH_TOZERO: ::libc::c_uint = 3;
pub const CV_THRESH_TOZERO_INV: ::libc::c_uint = 4;
pub const CV_THRESH_MASK: ::libc::c_uint = 7;
pub const CV_THRESH_OTSU: ::libc::c_uint = 8;
pub type Enum_Unnamed20 = ::libc::c_uint;
pub const CV_ADAPTIVE_THRESH_MEAN_C: ::libc::c_uint = 0;
pub const CV_ADAPTIVE_THRESH_GAUSSIAN_C: ::libc::c_uint = 1;
pub type Enum_Unnamed21 = ::libc::c_uint;
pub const CV_FLOODFILL_FIXED_RANGE: ::libc::c_uint = 65536;
pub const CV_FLOODFILL_MASK_ONLY: ::libc::c_uint = 131072;
pub type Enum_Unnamed22 = ::libc::c_int;
pub const CV_CANNY_L2_GRADIENT: ::libc::c_int = -2147483648;
pub type Enum_Unnamed23 = ::libc::c_uint;
pub const CV_HOUGH_STANDARD: ::libc::c_uint = 0;
pub const CV_HOUGH_PROBABILISTIC: ::libc::c_uint = 1;
pub const CV_HOUGH_MULTI_SCALE: ::libc::c_uint = 2;
pub const CV_HOUGH_GRADIENT: ::libc::c_uint = 3;
pub enum Struct_CvFeatureTree { }
pub enum Struct_CvLSH { }
pub enum Struct_CvLSHOperations { }

extern "C" {
    pub fn cvAcc(image: *const CvArr, sum: *mut CvArr, mask: *const CvArr)
     -> ();
    pub fn cvSquareAcc(image: *const CvArr, sqsum: *mut CvArr,
                       mask: *const CvArr) -> ();
    pub fn cvMultiplyAcc(image1: *const CvArr, image2: *const CvArr,
                         acc: *mut CvArr, mask: *const CvArr) -> ();
    pub fn cvRunningAvg(image: *const CvArr, acc: *mut CvArr,
                        alpha: ::libc::c_double, mask: *const CvArr) -> ();
    pub fn cvCopyMakeBorder(src: *const CvArr, dst: *mut CvArr,
                            offset: CvPoint, bordertype: ::libc::c_int,
                            value: CvScalar) -> ();
    pub fn cvSmooth(src: *const CvArr, dst: *mut CvArr,
                    smoothtype: ::libc::c_int, size1: ::libc::c_int,
                    size2: ::libc::c_int, sigma1: ::libc::c_double,
                    sigma2: ::libc::c_double) -> ();
    pub fn cvFilter2D(src: *const CvArr, dst: *mut CvArr,
                      kernel: *const CvMat, anchor: CvPoint) -> ();
    pub fn cvIntegral(image: *const CvArr, sum: *mut CvArr, sqsum: *mut CvArr,
                      tilted_sum: *mut CvArr) -> ();
    pub fn cvPyrDown(src: *const CvArr, dst: *mut CvArr,
                     filter: ::libc::c_int) -> ();
    pub fn cvPyrUp(src: *const CvArr, dst: *mut CvArr, filter: ::libc::c_int)
     -> ();
    pub fn cvCreatePyramid(img: *const CvArr, extra_layers: ::libc::c_int,
                           rate: ::libc::c_double, layer_sizes: *const CvSize,
                           bufarr: *mut CvArr, calc: ::libc::c_int,
                           filter: ::libc::c_int) -> *mut *mut CvMat;
    pub fn cvReleasePyramid(pyramid: *mut *mut *mut CvMat,
                            extra_layers: ::libc::c_int) -> ();
    pub fn cvPyrMeanShiftFiltering(src: *const CvArr, dst: *mut CvArr,
                                   sp: ::libc::c_double, sr: ::libc::c_double,
                                   max_level: ::libc::c_int,
                                   termcrit: CvTermCriteria) -> ();
    pub fn cvWatershed(image: *const CvArr, markers: *mut CvArr) -> ();
    pub fn cvSobel(src: *const CvArr, dst: *mut CvArr, xorder: ::libc::c_int,
                   yorder: ::libc::c_int, aperture_size: ::libc::c_int) -> ();
    pub fn cvLaplace(src: *const CvArr, dst: *mut CvArr,
                     aperture_size: ::libc::c_int) -> ();
    pub fn cvCvtColor(src: *const CvArr, dst: *mut CvArr, code: ::libc::c_int)
     -> ();
    pub fn cvResize(src: *const CvArr, dst: *mut CvArr,
                    interpolation: ::libc::c_int) -> ();
    pub fn cvWarpAffine(src: *const CvArr, dst: *mut CvArr,
                        map_matrix: *const CvMat, flags: ::libc::c_int,
                        fillval: CvScalar) -> ();
    pub fn cvGetAffineTransform(src: *const CvPoint2D32f,
                                dst: *const CvPoint2D32f,
                                map_matrix: *mut CvMat) -> *mut CvMat;
    pub fn cv2DRotationMatrix(center: CvPoint2D32f, angle: ::libc::c_double,
                              scale: ::libc::c_double, map_matrix: *mut CvMat)
     -> *mut CvMat;
    pub fn cvWarpPerspective(src: *const CvArr, dst: *mut CvArr,
                             map_matrix: *const CvMat, flags: ::libc::c_int,
                             fillval: CvScalar) -> ();
    pub fn cvGetPerspectiveTransform(src: *const CvPoint2D32f,
                                     dst: *const CvPoint2D32f,
                                     map_matrix: *mut CvMat) -> *mut CvMat;
    pub fn cvRemap(src: *const CvArr, dst: *mut CvArr, mapx: *const CvArr,
                   mapy: *const CvArr, flags: ::libc::c_int,
                   fillval: CvScalar) -> ();
    pub fn cvConvertMaps(mapx: *const CvArr, mapy: *const CvArr,
                         mapxy: *mut CvArr, mapalpha: *mut CvArr) -> ();
    pub fn cvLogPolar(src: *const CvArr, dst: *mut CvArr,
                      center: CvPoint2D32f, M: ::libc::c_double,
                      flags: ::libc::c_int) -> ();
    pub fn cvLinearPolar(src: *const CvArr, dst: *mut CvArr,
                         center: CvPoint2D32f, maxRadius: ::libc::c_double,
                         flags: ::libc::c_int) -> ();
    pub fn cvUndistort2(src: *const CvArr, dst: *mut CvArr,
                        camera_matrix: *const CvMat,
                        distortion_coeffs: *const CvMat,
                        new_camera_matrix: *const CvMat) -> ();
    pub fn cvInitUndistortMap(camera_matrix: *const CvMat,
                              distortion_coeffs: *const CvMat,
                              mapx: *mut CvArr, mapy: *mut CvArr) -> ();
    pub fn cvInitUndistortRectifyMap(camera_matrix: *const CvMat,
                                     dist_coeffs: *const CvMat,
                                     R: *const CvMat,
                                     new_camera_matrix: *const CvMat,
                                     mapx: *mut CvArr, mapy: *mut CvArr)
     -> ();
    pub fn cvUndistortPoints(src: *const CvMat, dst: *mut CvMat,
                             camera_matrix: *const CvMat,
                             dist_coeffs: *const CvMat, R: *const CvMat,
                             P: *const CvMat) -> ();
    pub fn cvCreateStructuringElementEx(cols: ::libc::c_int,
                                        rows: ::libc::c_int,
                                        anchor_x: ::libc::c_int,
                                        anchor_y: ::libc::c_int,
                                        shape: ::libc::c_int,
                                        values: *mut ::libc::c_int)
     -> *mut IplConvKernel;
    pub fn cvReleaseStructuringElement(element: *mut *mut IplConvKernel)
     -> ();
    pub fn cvErode(src: *const CvArr, dst: *mut CvArr,
                   element: *mut IplConvKernel, iterations: ::libc::c_int)
     -> ();
    pub fn cvDilate(src: *const CvArr, dst: *mut CvArr,
                    element: *mut IplConvKernel, iterations: ::libc::c_int)
     -> ();
    pub fn cvMorphologyEx(src: *const CvArr, dst: *mut CvArr,
                          temp: *mut CvArr, element: *mut IplConvKernel,
                          operation: ::libc::c_int, iterations: ::libc::c_int)
     -> ();
    pub fn cvMoments(arr: *const CvArr, moments: *mut CvMoments,
                     binary: ::libc::c_int) -> ();
    pub fn cvGetSpatialMoment(moments: *mut CvMoments, x_order: ::libc::c_int,
                              y_order: ::libc::c_int) -> ::libc::c_double;
    pub fn cvGetCentralMoment(moments: *mut CvMoments, x_order: ::libc::c_int,
                              y_order: ::libc::c_int) -> ::libc::c_double;
    pub fn cvGetNormalizedCentralMoment(moments: *mut CvMoments,
                                        x_order: ::libc::c_int,
                                        y_order: ::libc::c_int)
     -> ::libc::c_double;
    pub fn cvGetHuMoments(moments: *mut CvMoments,
                          hu_moments: *mut CvHuMoments) -> ();
    pub fn cvSampleLine(image: *const CvArr, pt1: CvPoint, pt2: CvPoint,
                        buffer: *mut ::libc::c_void,
                        connectivity: ::libc::c_int) -> ::libc::c_int;
    pub fn cvGetRectSubPix(src: *const CvArr, dst: *mut CvArr,
                           center: CvPoint2D32f) -> ();
    pub fn cvGetQuadrangleSubPix(src: *const CvArr, dst: *mut CvArr,
                                 map_matrix: *const CvMat) -> ();
    pub fn cvMatchTemplate(image: *const CvArr, templ: *const CvArr,
                           result: *mut CvArr, method: ::libc::c_int) -> ();
    pub fn cvCalcEMD2(signature1: *const CvArr, signature2: *const CvArr,
                      distance_type: ::libc::c_int,
                      distance_func: CvDistanceFunction,
                      cost_matrix: *const CvArr, flow: *mut CvArr,
                      lower_bound: *mut ::libc::c_float,
                      userdata: *mut ::libc::c_void) -> ::libc::c_float;
    pub fn cvFindContours(image: *mut CvArr, storage: *mut CvMemStorage,
                          first_contour: *mut *mut CvSeq,
                          header_size: ::libc::c_int, mode: ::libc::c_int,
                          method: ::libc::c_int, offset: CvPoint)
     -> ::libc::c_int;
    pub fn cvStartFindContours(image: *mut CvArr, storage: *mut CvMemStorage,
                               header_size: ::libc::c_int,
                               mode: ::libc::c_int, method: ::libc::c_int,
                               offset: CvPoint) -> CvContourScanner;
    pub fn cvFindNextContour(scanner: CvContourScanner) -> *mut CvSeq;
    pub fn cvSubstituteContour(scanner: CvContourScanner,
                               new_contour: *mut CvSeq) -> ();
    pub fn cvEndFindContours(scanner: *mut CvContourScanner) -> *mut CvSeq;
    pub fn cvApproxChains(src_seq: *mut CvSeq, storage: *mut CvMemStorage,
                          method: ::libc::c_int, parameter: ::libc::c_double,
                          minimal_perimeter: ::libc::c_int,
                          recursive: ::libc::c_int) -> *mut CvSeq;
    pub fn cvStartReadChainPoints(chain: *mut CvChain,
                                  reader: *mut CvChainPtReader) -> ();
    pub fn cvReadChainPoint(reader: *mut CvChainPtReader) -> CvPoint;
    pub fn cvApproxPoly(src_seq: *const ::libc::c_void,
                        header_size: ::libc::c_int,
                        storage: *mut CvMemStorage, method: ::libc::c_int,
                        eps: ::libc::c_double, recursive: ::libc::c_int)
     -> *mut CvSeq;
    pub fn cvArcLength(curve: *const ::libc::c_void, slice: CvSlice,
                       is_closed: ::libc::c_int) -> ::libc::c_double;
    pub fn cvBoundingRect(points: *mut CvArr, update: ::libc::c_int)
     -> CvRect;
    pub fn cvContourArea(contour: *const CvArr, slice: CvSlice,
                         oriented: ::libc::c_int) -> ::libc::c_double;
    pub fn cvMinAreaRect2(points: *const CvArr, storage: *mut CvMemStorage)
     -> CvBox2D;
    pub fn cvMinEnclosingCircle(points: *const CvArr,
                                center: *mut CvPoint2D32f,
                                radius: *mut ::libc::c_float)
     -> ::libc::c_int;
    pub fn cvMatchShapes(object1: *const ::libc::c_void,
                         object2: *const ::libc::c_void,
                         method: ::libc::c_int, parameter: ::libc::c_double)
     -> ::libc::c_double;
    pub fn cvConvexHull2(input: *const CvArr,
                         hull_storage: *mut ::libc::c_void,
                         orientation: ::libc::c_int,
                         return_points: ::libc::c_int) -> *mut CvSeq;
    pub fn cvCheckContourConvexity(contour: *const CvArr) -> ::libc::c_int;
    pub fn cvConvexityDefects(contour: *const CvArr, convexhull: *const CvArr,
                              storage: *mut CvMemStorage) -> *mut CvSeq;
    pub fn cvFitEllipse2(points: *const CvArr) -> CvBox2D;
    pub fn cvMaxRect(rect1: *const CvRect, rect2: *const CvRect) -> CvRect;
    pub fn cvBoxPoints(_box: CvBox2D, pt: *mut CvPoint2D32f) -> ();
    pub fn cvPointSeqFromMat(seq_kind: ::libc::c_int, mat: *const CvArr,
                             contour_header: *mut CvContour,
                             block: *mut CvSeqBlock) -> *mut CvSeq;
    pub fn cvPointPolygonTest(contour: *const CvArr, pt: CvPoint2D32f,
                              measure_dist: ::libc::c_int)
     -> ::libc::c_double;
    pub fn cvCreateHist(dims: ::libc::c_int, sizes: *mut ::libc::c_int,
                        _type: ::libc::c_int,
                        ranges: *mut *mut ::libc::c_float,
                        uniform: ::libc::c_int) -> *mut CvHistogram;
    pub fn cvSetHistBinRanges(hist: *mut CvHistogram,
                              ranges: *mut *mut ::libc::c_float,
                              uniform: ::libc::c_int) -> ();
    pub fn cvMakeHistHeaderForArray(dims: ::libc::c_int,
                                    sizes: *mut ::libc::c_int,
                                    hist: *mut CvHistogram,
                                    data: *mut ::libc::c_float,
                                    ranges: *mut *mut ::libc::c_float,
                                    uniform: ::libc::c_int)
     -> *mut CvHistogram;
    pub fn cvReleaseHist(hist: *mut *mut CvHistogram) -> ();
    pub fn cvClearHist(hist: *mut CvHistogram) -> ();
    pub fn cvGetMinMaxHistValue(hist: *const CvHistogram,
                                min_value: *mut ::libc::c_float,
                                max_value: *mut ::libc::c_float,
                                min_idx: *mut ::libc::c_int,
                                max_idx: *mut ::libc::c_int) -> ();
    pub fn cvNormalizeHist(hist: *mut CvHistogram, factor: ::libc::c_double)
     -> ();
    pub fn cvThreshHist(hist: *mut CvHistogram, threshold: ::libc::c_double)
     -> ();
    pub fn cvCompareHist(hist1: *const CvHistogram, hist2: *const CvHistogram,
                         method: ::libc::c_int) -> ::libc::c_double;
    pub fn cvCopyHist(src: *const CvHistogram, dst: *mut *mut CvHistogram)
     -> ();
    pub fn cvCalcBayesianProb(src: *mut *mut CvHistogram,
                              number: ::libc::c_int,
                              dst: *mut *mut CvHistogram) -> ();
    pub fn cvCalcArrHist(arr: *mut *mut CvArr, hist: *mut CvHistogram,
                         accumulate: ::libc::c_int, mask: *const CvArr) -> ();
    pub fn cvCalcArrBackProject(image: *mut *mut CvArr, dst: *mut CvArr,
                                hist: *const CvHistogram) -> ();
    pub fn cvCalcArrBackProjectPatch(image: *mut *mut CvArr, dst: *mut CvArr,
                                     range: CvSize, hist: *mut CvHistogram,
                                     method: ::libc::c_int,
                                     factor: ::libc::c_double) -> ();
    pub fn cvCalcProbDensity(hist1: *const CvHistogram,
                             hist2: *const CvHistogram,
                             dst_hist: *mut CvHistogram,
                             scale: ::libc::c_double) -> ();
    pub fn cvEqualizeHist(src: *const CvArr, dst: *mut CvArr) -> ();
    pub fn cvDistTransform(src: *const CvArr, dst: *mut CvArr,
                           distance_type: ::libc::c_int,
                           mask_size: ::libc::c_int,
                           mask: *const ::libc::c_float, labels: *mut CvArr,
                           labelType: ::libc::c_int) -> ();
    pub fn cvThreshold(src: *const CvArr, dst: *mut CvArr,
                       threshold: ::libc::c_double,
                       max_value: ::libc::c_double,
                       threshold_type: ::libc::c_int) -> ::libc::c_double;
    pub fn cvAdaptiveThreshold(src: *const CvArr, dst: *mut CvArr,
                               max_value: ::libc::c_double,
                               adaptive_method: ::libc::c_int,
                               threshold_type: ::libc::c_int,
                               block_size: ::libc::c_int,
                               param1: ::libc::c_double) -> ();
    pub fn cvFloodFill(image: *mut CvArr, seed_point: CvPoint,
                       new_val: CvScalar, lo_diff: CvScalar,
                       up_diff: CvScalar, comp: *mut CvConnectedComp,
                       flags: ::libc::c_int, mask: *mut CvArr) -> ();
    pub fn cvCanny(image: *const CvArr, edges: *mut CvArr,
                   threshold1: ::libc::c_double, threshold2: ::libc::c_double,
                   aperture_size: ::libc::c_int) -> ();
    pub fn cvPreCornerDetect(image: *const CvArr, corners: *mut CvArr,
                             aperture_size: ::libc::c_int) -> ();
    pub fn cvCornerEigenValsAndVecs(image: *const CvArr, eigenvv: *mut CvArr,
                                    block_size: ::libc::c_int,
                                    aperture_size: ::libc::c_int) -> ();
    pub fn cvCornerMinEigenVal(image: *const CvArr, eigenval: *mut CvArr,
                               block_size: ::libc::c_int,
                               aperture_size: ::libc::c_int) -> ();
    pub fn cvCornerHarris(image: *const CvArr, harris_response: *mut CvArr,
                          block_size: ::libc::c_int,
                          aperture_size: ::libc::c_int, k: ::libc::c_double)
     -> ();
    pub fn cvFindCornerSubPix(image: *const CvArr, corners: *mut CvPoint2D32f,
                              count: ::libc::c_int, win: CvSize,
                              zero_zone: CvSize, criteria: CvTermCriteria)
     -> ();
    pub fn cvGoodFeaturesToTrack(image: *const CvArr, eig_image: *mut CvArr,
                                 temp_image: *mut CvArr,
                                 corners: *mut CvPoint2D32f,
                                 corner_count: *mut ::libc::c_int,
                                 quality_level: ::libc::c_double,
                                 min_distance: ::libc::c_double,
                                 mask: *const CvArr,
                                 block_size: ::libc::c_int,
                                 use_harris: ::libc::c_int,
                                 k: ::libc::c_double) -> ();
    pub fn cvHoughLines2(image: *mut CvArr, line_storage: *mut ::libc::c_void,
                         method: ::libc::c_int, rho: ::libc::c_double,
                         theta: ::libc::c_double, threshold: ::libc::c_int,
                         param1: ::libc::c_double, param2: ::libc::c_double)
     -> *mut CvSeq;
    pub fn cvHoughCircles(image: *mut CvArr,
                          circle_storage: *mut ::libc::c_void,
                          method: ::libc::c_int, dp: ::libc::c_double,
                          min_dist: ::libc::c_double,
                          param1: ::libc::c_double, param2: ::libc::c_double,
                          min_radius: ::libc::c_int,
                          max_radius: ::libc::c_int) -> *mut CvSeq;
    pub fn cvFitLine(points: *const CvArr, dist_type: ::libc::c_int,
                     param: ::libc::c_double, reps: ::libc::c_double,
                     aeps: ::libc::c_double, line: *mut ::libc::c_float)
     -> ();
}
