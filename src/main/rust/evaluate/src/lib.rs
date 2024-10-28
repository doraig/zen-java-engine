use robusta_jni::bridge;

/**

*/
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}



#[bridge]
mod jni {
    use robusta_jni::convert::{
        Field, IntoJavaValue, Signature, TryFromJavaValue, TryIntoJavaValue,
    };
    use robusta_jni::jni::errors::Error as JniError;
    use robusta_jni::jni::errors::Result as JniResult;
    use robusta_jni::jni::objects::AutoLocal;
    use robusta_jni::jni::JNIEnv;
    use zen_engine::{DecisionEngine};
    use zen_engine::model::DecisionContent;
    use futures::executor;
    use std::fs;
    use std::string::ToString;
    use serde_json::{json, Value, from_str};
    use uuid::Uuid;

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(io.doraig)]
    pub struct ZenEnginImpl<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>,
        #[field]
        jniStatus: Field<'env, 'borrow, String>,
    }

    impl<'env: 'borrow, 'borrow> ZenEnginImpl<'env, 'borrow> {

        #[constructor]
        pub extern "java" fn new(env: &'borrow JNIEnv<'env>) -> JniResult<Self> {}

        #[call_type(safe(
            exception_class = "java.lang.IllegalArgumentException",
            message = "something bad happened"
        ))]
        pub extern "jni" fn testJniCall(mut self, env: &'borrow JNIEnv<'env>, mut listValue: Vec<i32>, newValue: i32) -> JniResult<Vec<String>> {
            listValue.push(newValue);
            match Self::new(env) {
                Ok(instance) => instance.javaAdd(env, 2, 3).unwrap_or(-1),
                Err(_e) => return Err(JniError::JavaException),
            };
            match self.javaAdd(env, 1, 2) {
                Ok(_) => (),
                Err(_e) => return Err(JniError::JavaException),
            }
            match self.jniStatus.set("OK".into()) {
                Ok(_) => (),
                Err(_e) => return Err(JniError::JavaException),
            }

            Ok(listValue.iter().map(ToString::to_string).collect())
        }

        pub extern "java" fn javaAdd(&self, _env: &JNIEnv, i: i32, u: i32) -> JniResult<i32> {}

        #[call_type(safe(
            exception_class = "java.lang.Exception",
            message = "Evaluation failed unexpectedly"
        ))]
        pub extern "jni" fn jniEvaluate(self, _env: &'borrow JNIEnv<'env>, requestContent: String, ruleFile: String) -> JniResult<String> {
            let result = executor::block_on(Self::execute_rule(requestContent.as_str(), ruleFile.as_str()));

            match result {
                Ok(value) => Ok(value),
                Err(_e) => return Err(JniError::JavaException),
            }
        }

        async fn execute_rule(requestContent: &str, ruleFile: &str) -> Result<String, Box<dyn std::error::Error>> {
            let data = fs::read_to_string(ruleFile)?;
            let decision_content: DecisionContent = serde_json::from_str(data.as_str()).unwrap();
            let engine = DecisionEngine::default();
            let decision = engine.create_decision(decision_content.into());

            let data = match from_str::<Value>(requestContent) {
                Ok(value) => value,
                Err(_e) => json!({"errors": [
                {
                  "id": Uuid::new_v4().to_string(),
                  "code": "9001",
                  "status": "500",
                  "title": "Unexpected error",
                  "detail": "Unexpected error"
                }
              ]})
            };

            match decision.evaluate(data.into()).await {
                Ok(result) => Ok(result.result.to_string()),
                Err(e) => Err(Box::new(e)),
            }
        }

    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
