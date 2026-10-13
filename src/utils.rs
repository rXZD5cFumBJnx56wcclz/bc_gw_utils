use bc_utils_lg::prelude::*;

pub trait GwSrc<T> {
    // examples: with buffer
    fn push_map_settings(&mut self, map: &MAP<&str, Vec<T>>, used: &[SettingsUsedString]);
}

#[macro_export]
macro_rules! impl_gw_src_single {
    ($wrapper:ty, $inner_type:ty) => {
        impl $crate::utils::GwSrc<$inner_type> for $wrapper {
            fn push_map_settings(
                &mut self,
                map: &MAP<&str, Vec<$inner_type>>,
                used: &[SettingsUsedString],
            ) {
                for used_ind in used {
                    let sk = &map[used_ind.key.as_str()];
                    self.0.push(sk[sk.len() - used_ind.sub_from_last_i - 1]);
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_gw_src_vec {
    ($wrapper:ty, $inner_type:ty) => {
        impl $crate::utils::GwSrc<$inner_type> for $wrapper {
            fn push_map_settings(
                &mut self,
                map: &MAP<&str, Vec<$inner_type>>,
                used: &[SettingsUsedString],
            ) {
                for used_ind in used {
                    let sk = &map[used_ind.key.as_str()];
                    self.0
                        .push(sk[..sk.len() - used_ind.sub_from_last_i].to_vec());
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_deref {
    ($wrapper:ty, $inner_type:ty) => {
        impl std::ops::Deref for $wrapper {
            type Target = $inner_type;
            fn deref(&self) -> &$inner_type {
                &self.0
            }
        }
    };
}
