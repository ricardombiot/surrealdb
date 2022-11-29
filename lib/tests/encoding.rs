mod parse;
use parse::Parse;
use surrealdb::sql::Value;
use surrealdb::Datastore;
use surrealdb::Error;
use surrealdb::Session;


#[tokio::test]
async fn testing_encoding() -> Result<(), Error> {
	let sql = "
		CREATE message:nihao SET text = '你好';
        CREATE message:namaste SET text = 'नमस्ते';
	";
	let dbs = Datastore::new("memory").await?;
	let ses = Session::for_kv().with_ns("test").with_db("test");
	let res = &mut dbs.execute(&sql, &ses, None, false).await?;
	assert_eq!(res.len(), 2);
	//
	let tmp = res.remove(0).result?;
	let val = Value::parse(
		"[
			{
				id: message:nihao,
				text: '你好'
			}
		]",
	);
	assert_eq!(tmp, val);
	//
	let tmp = res.remove(0).result?;
	let val = Value::parse(
		"[
			{
				id: message:namaste,
				text: 'नमस्ते'
			}
		]",
	);
	assert_eq!(tmp, val);

	Ok(())

}



//https://github.com/surrealdb/surrealdb/issues/1500
#[tokio::test]
async fn testing_encoding_german_umlauts() -> Result<(), Error> {
	let sql = "
    create malicous_record:umlauts SET boom = 'LÖL';
	";
	let dbs = Datastore::new("memory").await?;
	let ses = Session::for_kv().with_ns("test").with_db("test");
	let res = &mut dbs.execute(&sql, &ses, None, false).await?;
	assert_eq!(res.len(), 1);
	//
	let tmp = res.remove(0).result?;
	let val = Value::parse(
		"[
			{
				id: malicous_record:umlauts,
				boom: 'LÖL'
			}
		]",
	);
    assert_eq!(tmp, val);
	Ok(())

}

