use std::error::Error;
use context::Context;
use pool_blockchain::BlockchainPool;
use pool_sync::Run;
use pool_transactions::TransactionPool;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use crate::transaction::TransactionPost;

pub struct ApiState {
    pub blockchain: BlockchainPool,
    pub transactions: TransactionPool,
}

pub struct Api {
    port: u16,
    blockchain: BlockchainPool,
    transactions: TransactionPool,
}

impl Api {
    pub fn new(context: &Context) -> Api {
        Api {
            port: context.config.port,
            blockchain: context.blockchain.clone(),
            transactions: context.transactions.clone(),
        }
    }

    pub fn start(&self) -> Result<()> {

        let blockchain = self.blockchain.clone();
        let transactions = self.transactions.clone();

        server_start(self.port, blockchain, transactions)
    }

    pub fn print_blockchain(&self) {
        println!("{:?}", self.blockchain.blockchain);
    }

}


impl Run for Api {
    fn run(&self) -> Result<()> {
        self.start()
    }
}

#[actix_web::main]
async fn server_start(port: u16, blockchain: BlockchainPool, transactions: TransactionPool) -> Result<()> {
    println!("Api is running...");
    let url = format!("0.0.0.0:{}", port);
    let api_state = web::Data::new(ApiState {
        blockchain,
        transactions,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(api_state.clone())
            .service(get_blocks)
            .service(get_block)
            .service(get_transactions)
            .service(get_transaction)
            .service(post_transaction)
    }).bind(url)
        .unwrap()
        .run()
        .await?;
    Ok(())
}

#[actix_web::get("/blocks")]
async fn get_blocks(state: web::Data<ApiState>) -> impl Responder {
    let blockchain = &state.blockchain;
    let blocks = blockchain.get_all_blocks();
    HttpResponse::Ok().json(blocks)
}

#[actix_web::get("/block/{id}")]
async fn get_block(state: web::Data<ApiState>, id: web::Path<u64>) -> impl Responder {
    let blockchain = &state.blockchain;
    match blockchain.get_block(id.into_inner()) {
        Ok(block) => {
            HttpResponse::Ok().json(block)
        }
        Err(err) => {
            HttpResponse::BadRequest().json(err.description())
        }
    }
}

#[actix_web::get("/transactions")]
async fn get_transactions(state: web::Data<ApiState>) -> impl Responder {
    let transactions = &state.transactions;
    let transactions = transactions.get_all_transactions();
    HttpResponse::Ok().json(transactions)
}

#[actix_web::get("/transaction/{id}")]
async fn get_transaction(state: web::Data<ApiState>, id: web::Path<u64>) -> impl Responder {
    let transactions = &state.transactions;
    match transactions.get_transaction(id.into_inner()) {
        Ok(transaction) => {
            HttpResponse::Ok().json(transaction)
        }
        Err(err) => {
            HttpResponse::BadRequest().json(err.description())
        }
    }
}

#[actix_web::post("/transaction")]
async fn post_transaction(state: web::Data<ApiState>, info: web::Json<TransactionPost>) -> impl Responder {
    let transactions = &state.transactions;
    let transaction = info.into_transaction();
    transactions.add_transaction(transaction.clone());
    HttpResponse::Ok().json(transaction)
}