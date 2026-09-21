use bc_utils::other::{procedure_used, transpose_set, vec_len_sync_set};
use bc_utils_lg::prelude::*;

use crate::{impl_deref, impl_gw_src_single, impl_gw_src_vec};

#[derive(Debug, Default, Clone)]
pub struct SrcGw(pub Vec<Vec<f64>>);


impl SrcGw {
    pub fn push_vec(&mut self, src: &[Vec<f64>], used: &[SETTINGS_USED_USIZE]) {
        for used_src_el in used {
            self.0.push({
                let sk = &src[used_src_el.index];
                sk[..sk.len() - used_src_el.sub_from_last_i].to_vec()
            });
        }
    }

    pub fn push_map(&mut self, map: &MAP<&str, Vec<f64>>, used: &[String]) {
        for used_ind in used {
            self.0.push(map[used_ind.as_str()].clone());
        }
    }

    pub fn len_sync(&mut self) {
        vec_len_sync_set(&mut self.0);
    }

    pub fn transpose(&mut self) {
        transpose_set(&mut self.0)
    }

    pub fn procedure_used(&mut self, procedure: &[usize]) {
        self.0 = procedure_used(std::mem::take(&mut self.0), procedure);
    }

    pub fn all_check(&mut self, procedure: &[usize]) {
        if !procedure.is_empty() {
            self.procedure_used(procedure);
        }
        if !self.0.is_empty() {
            self.len_sync();
            self.transpose();
        }
    }
}

impl_gw_src_vec!(SrcGw, f64);
impl_deref!(SrcGw, Vec<Vec<f64>>);

#[derive(Debug, Default, Clone)]
pub struct SrcGwSeries(pub Vec<f64>);

impl SrcGwSeries {
    pub fn push_vec(&mut self, src: &[Vec<f64>], used: &[SETTINGS_USED_USIZE]) {
        for used_src_el in used {
            self.0.push({
                let sk = &src[used_src_el.index];
                sk[sk.len() - used_src_el.sub_from_last_i - 1]
            });
        }
    }

    pub fn push_map(&mut self, map: &MAP<&str, f64>, used: &[String]) {
        for used_ind in used {
            self.0.push(map[used_ind.as_str()].clone());
        }
    }

    pub fn procedure_used(&mut self, procedure: &[usize]) {
        self.0 = procedure_used(std::mem::take(&mut self.0), procedure);
    }

    pub fn all_check(&mut self, procedure: &[usize]) {
        if !procedure.is_empty() {
            self.procedure_used(procedure);
        }
    }
}

impl_gw_src_single!(SrcGwSeries, f64);
impl_deref!(SrcGwSeries, Vec<f64>);

#[cfg(test)]
mod test {
    use super::*;
    use bc_test_kit::prelude::*;
    use bc_utils::other::transpose;
    use bc_utils_lg::test_state::prelude::*;
    
    #[test]
    fn get_src_res_1() {
        let mut src = SrcGw::default();
        src.push_vec(&SRC_TRANSPOSE, &INDICATIONS["rma_1"].used_src);
        src.all_check(&INDICATIONS["rma_1"].procedure_used);
        assert_eq_pr!(
            &*src,
            &transpose(vec![CLOSE[..49].to_vec()]),
        )
    }

    #[test]
    fn get_src_series_res_1() {
        let mut src = SrcGwSeries::default();
        src.push_vec(&SRC_TRANSPOSE, &INDICATIONS["rma_1"].used_src);
        src.all_check(&INDICATIONS["rma_1"].procedure_used);
        assert_eq_pr!(
            &*src,
            &vec![CLOSE[48]],
        )
    }

    #[test]
    fn get_src_series_res_2() {
        let mut src = SrcGwSeries::default();
        src.push_vec(&SRC_TRANSPOSE, &SIGNALS_TRAIN["mm_1"].used_src);
        src.push_map(&MAP::from_iter([("rma_1", 1.)]),&SIGNALS_TRAIN["mm_1"].used_ind);
        src.all_check(&SIGNALS_TRAIN["mm_1"].procedure_used_src);
        assert_eq_pr!(
            &*src,
            &vec![SRC_EL1[1], 1.,]
        )
    }
}