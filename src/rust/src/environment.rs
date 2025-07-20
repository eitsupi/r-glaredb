use std::sync::{Arc, Mutex};

use glaredb::ext::datafusion::datasource::{MemTable, TableProvider};
use glaredb::ext::EnvironmentReader;
use savvy::ffi::SEXP;
use savvy::protect::{insert_to_preserved_list, release_from_preserved_list};
use savvy::{EnvironmentSexp, Sexp};

use crate::execution::RGlareDbExecutionOutput;
use crate::table::RGlareDbTable;

// TODO
struct UnsafeToken(SEXP);
unsafe impl std::marker::Send for UnsafeToken {}
unsafe impl std::marker::Sync for UnsafeToken {}
struct UnsafeEnvironmentSexp(EnvironmentSexp);
unsafe impl std::marker::Send for UnsafeEnvironmentSexp {}
unsafe impl std::marker::Sync for UnsafeEnvironmentSexp {}

pub struct REnvironmentReader {
    env: Arc<Mutex<UnsafeEnvironmentSexp>>,
    token: UnsafeToken,
}

impl REnvironmentReader {
    pub(crate) fn new(env: EnvironmentSexp) -> Self {
        let token = insert_to_preserved_list(env.inner());
        Self {
            env: Arc::new(Mutex::new(UnsafeEnvironmentSexp(env))),
            token: UnsafeToken(token),
        }
    }
}

impl Drop for REnvironmentReader {
    fn drop(&mut self) {
        release_from_preserved_list(self.token.0)
    }
}

impl EnvironmentReader for REnvironmentReader {
    fn resolve_table(
        &self,
        name: &str,
    ) -> Result<Option<Arc<dyn TableProvider>>, Box<dyn std::error::Error + Send + Sync>> {
        let env = (*self.env).lock().unwrap();
        let Ok(obj) = env
            .0
            .get(name)
            .map_err(|e| e.to_string())?
            .ok_or("Not Found")
        else {
            return Ok(None);
        };
        let classes = obj.get_class().unwrap_or(vec![]);

        if classes.iter().any(|&s| s == "RGlareDbExecutionOutput") {
            let sexp = EnvironmentSexp::try_from(obj)
                .unwrap()
                .get(".ptr")
                .expect("RGlareDbExecutionOutput should have .ptr")
                .ok_or("Not found")?;
            let exec = <&RGlareDbExecutionOutput>::try_from(sexp).unwrap().clone();

            return Ok(Some(Arc::new(exec) as Arc<dyn TableProvider>));
        }

        if classes.iter().any(|&s| s == "RGlareDbTable") {
            let sexp = EnvironmentSexp::try_from(obj)
                .unwrap()
                .get(".ptr")
                .expect("RGlareDbTable should have .ptr")
                .ok_or("Not found")?;
            let table: MemTable = <&RGlareDbTable>::try_from(sexp).unwrap().try_into()?;

            return Ok(Some(Arc::new(table) as Arc<dyn TableProvider>));
        }

        if classes
            .iter()
            .any(|&s| s == "RPolarsDataFrame" || s == "ArrowTabular" || s == "polars_data_frame")
        {
            let func = savvy::FunctionSexp::try_from(savvy::Sexp(
                savvy::eval_parse_text(r#"utils::getFromNamespace("as_glaredb_table", "glaredb")"#)
                    .unwrap()
                    .inner(),
            ))
            .unwrap();
            let mut args = savvy::FunctionArgs::new();
            let _ = args.add("x", obj);
            let wrapper_sexp: EnvironmentSexp = Sexp::try_from(func.call(args).unwrap())
                .unwrap()
                .try_into()
                .unwrap();
            let sexp = wrapper_sexp
                .get(".ptr")
                .expect("RGlareDbTable should have .ptr")
                .ok_or("Not found")?;

            let table: MemTable = <&RGlareDbTable>::try_from(sexp).unwrap().try_into()?;

            return Ok(Some(Arc::new(table) as Arc<dyn TableProvider>));
        }

        Ok(None)
    }
}
