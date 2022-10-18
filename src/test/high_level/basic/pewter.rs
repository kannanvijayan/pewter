
use crate::{ Pewter, PewterConfig };

#[test]
fn test_pewter_basics() {
  tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .expect("Failed to build tokio runtime")
    .block_on(async {
      let config = PewterConfig::default();
      let pewter = Pewter::new(config).await
        .expect("Failed to create pewter.");
      pewter.declare(move |decl| {
        let sh_add_u32 = decl.shader_file("add_u32", |decl_sh| {

          decl_sh.function("add_u32_u32", |decl_sf| {
            let arg0 = decl_sf.use_arg::<u32>("arg0");
            let arg1 = decl_sf.use_arg::<u32>("arg1");
            decl_sf.return_stmt(arg0 + arg1);
          });

          decl_sh.entry_function_1d("mul_coord_by_3", |decl_sh| {
          });
        });

        let text = decl.shader_file_text(&sh_add_u32);
        eprint!("TEXT:\n{}", text);
        Ok(())
      }).expect("Failed to declare resources.");
    });
}