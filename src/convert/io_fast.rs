use super::Converter;
use crate::structs::{FastIOJudgeSpec, JudgeSpec, JudgeSpecs, Problem};
use async_trait::async_trait;
use std::fs;
use std::path::Path;

pub struct IOFastConverter {}

#[async_trait]
impl Converter for IOFastConverter {
    async fn convert(&self, problem: &Problem, dir: &Path) -> JudgeSpecs {
        let mut specs = Vec::new();

        let max_cost = problem.policy.iter().map(|p| p.budget).max().unwrap_or(0);
        let max_memory = problem.policy.iter().map(|p| p.memory).max().unwrap_or(0);

        for testcase in &problem.testcase {
            let mut spec = FastIOJudgeSpec {
                input: None,
                input_url: None,
                input_auth: None,
                output_hash: "".to_string(),
                cost: max_cost,
                memory: max_memory.try_into().unwrap_or(0),
            };

            if let Some(stdin) = &testcase.stdin {
                spec.input = Some(stdin.clone());
            }

            if let Some(stdin_file) = &testcase.stdin_file {
                spec.input_url = Some(stdin_file.clone());
            }

            if let Some(stdout) = &testcase.stdout {
                spec.output_hash = sha256::digest(stdout.trim().as_bytes());
            } else if let Some(stdout_file) = &testcase.stdout_file {
                let is_remote =
                    stdout_file.starts_with("http://") || stdout_file.starts_with("https://");
                if is_remote {
                    let content = reqwest::get(stdout_file)
                        .await
                        .expect("Failed to download stdout file")
                        .text()
                        .await
                        .expect("Failed to read stdout file");
                    spec.output_hash = sha256::digest(content.trim().as_bytes());
                } else {
                    let content = fs::read_to_string(dir.join(stdout_file))
                        .expect("Failed to read stdout file");
                    spec.output_hash = sha256::digest(content.trim().as_bytes());
                }
            } else {
                spec.output_hash = sha256::digest("".as_bytes());
            }

            specs.push(JudgeSpec::IOFast(spec));
        }

        JudgeSpecs { specs }
    }
}
