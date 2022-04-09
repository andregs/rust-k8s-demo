# README
## Rust App on Kubernetes via Skaffold & Cloud Native Buildpacks

https://user-images.githubusercontent.com/712092/162587571-dae3c849-0b28-4b50-b220-ed81d805639e.mp4

1. Create simple hello world app.
2. Create app deployment manifest:

```shell
mkdir k8s
kubectl create deployment ferris-k8s --image=andregs/ferris-k8s --dry-run=client -o yaml > k8s/deployment.yaml
```

3. Add skaffold support via:

```shell
skaffold init -k ./k8s/*.yaml --skip-build
```

4. Create a `Procfile` with the path of your release binary.
5. Add the whole build section to the generated yaml file, specifying how skaffold should build your app image.
6. Build & deploy to k8s with skaffold:

```shell
minikube start
skaffold dev --port-forward
```

Skaffold will build your app, publish it to kubernetes, watch fs, live reload it whenever you change the code and cleanup after itself when you ^C it.
