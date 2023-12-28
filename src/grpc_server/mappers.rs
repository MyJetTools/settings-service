use crate::{domains_grpc::NginxConfigGrpcModel, my_no_sql::NginxSetupMyNoSqlEntity};

impl Into<NginxConfigGrpcModel> for NginxSetupMyNoSqlEntity {
    fn into(self) -> NginxConfigGrpcModel {
        NginxConfigGrpcModel {
            protected_with_ca: self.protected_with_ca,
            template: self.use_template,
            routes: self
                .rotes
                .into_iter()
                .map(|itm| crate::domains_grpc::NginxRouteGrpcModel {
                    path: itm.path,
                    proxy_to: itm.proxy_to,
                    template: itm.use_template,
                })
                .collect(),
        }
    }
}
