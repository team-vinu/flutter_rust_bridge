use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::type_dart_generator_struct;
use crate::utils::BlockIndex;

type_dart_generator_struct!(TypeOptionalGenerator, IrTypeOptional);

impl TypeDartGeneratorTrait for TypeOptionalGenerator<'_> {
    fn api2wire_body(&self, _block_index: BlockIndex) -> Option<String> {
        Some(format!(
            "return raw.match(
                (r) => _api2wire_{}(r),
                () => ffi.nullptr,
            );",
            self.ir.inner.safe_ident()
        ))
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        if !self.ir.needs_initialization() || self.ir.is_list() {
            return None;
        }
        Some(format!(
            "if (apiObj != null) _api_fill_to_wire_{}(apiObj, wireObj);",
            self.ir.inner.safe_ident()
        ))
    }

    fn wire2api_body(&self) -> String {
        format!(
            "return raw == null ? fp.none() : fp.some(_wire2api_{}(raw));",
            self.ir.inner.safe_ident()
        )
    }
}
