//! Runs a short-lived container with explicit cleanup using Bollard.
//!
//! A container runs a program in an isolated process with its own file system,
//! built from an image. Docker is the common runtime. Programs use its API to
//! start throwaway services in tests. Always remove the container afterwards,
//! or stopped containers pile up on the machine.

// ANCHOR: example
use anyhow::Context;
use bollard::Docker;
use bollard::models::ContainerCreateBody;
use bollard::query_parameters::CreateContainerOptions;
use bollard::query_parameters::CreateImageOptionsBuilder;
use bollard::query_parameters::ListContainersOptionsBuilder;
use bollard::query_parameters::RemoveContainerOptions;
use bollard::query_parameters::StartContainerOptions;
use bollard::query_parameters::WaitContainerOptions;
use futures_util::TryStreamExt;

/// Lists existing containers and runs Alpine's `true` command in a new one.
async fn main() -> anyhow::Result<()> {
    let docker = Docker::connect_with_local_defaults()?;
    let containers = docker
        .list_containers(Some(
            ListContainersOptionsBuilder::new().all(true).build(),
        ))
        .await?;
    println!("{} existing containers", containers.len());
    let image = "alpine:3.23";
    let mut pull = docker.create_image(
        Some(CreateImageOptionsBuilder::new().from_image(image).build()),
        None,
        None,
    );
    while pull.try_next().await?.is_some() {}
    let created = docker
        .create_container(
            None::<CreateContainerOptions>,
            ContainerCreateBody {
                image: Some(image.to_owned()),
                cmd: Some(vec!["true".to_owned()]),
                ..Default::default()
            },
        )
        .await?;
    // Docker assigns the name, so parallel runs cannot collide by name.
    let outcome = async {
        docker
            .start_container(&created.id, None::<StartContainerOptions>)
            .await?;
        let status = docker
            .wait_container(&created.id, None::<WaitContainerOptions>)
            .try_next()
            .await?
            .context("Docker returned no container exit status")?;
        anyhow::ensure!(
            status.status_code == 0,
            "container exited with {}",
            status.status_code
        );
        Ok::<_, anyhow::Error>(())
    }
    .await;
    let cleanup = docker
        .remove_container(
            &created.id,
            Some(RemoveContainerOptions {
                force: true,
                ..Default::default()
            }),
        )
        .await;
    match (outcome, cleanup) {
        (Ok(()), Ok(())) => Ok(()),
        (Err(error), Ok(())) => Err(error),
        (Ok(()), Err(error)) => Err(error.into()),
        (Err(error), Err(cleanup)) => Err(anyhow::anyhow!(
            "container failed: {error}; cleanup also failed: {cleanup}"
        )),
    }
}
// ANCHOR_END: example

/// Runs the Bollard recipe against a local Docker daemon.
pub async fn run() -> anyhow::Result<()> {
    main().await
}

#[cfg(test)]
mod tests {
    /// Verifies container execution, exit status, and successful removal.
    #[tokio::test]
    #[ignore = "needs a Docker daemon"]
    async fn runs_and_removes_a_container() -> anyhow::Result<()> {
        let result = super::main().await;
        assert!(result.is_ok(), "container run failed: {result:?}");
        result
    }
}
