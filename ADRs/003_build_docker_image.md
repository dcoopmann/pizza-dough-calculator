# Build docker image
To make the service deployable wrap it in docker

Things to consider:
- Docker file and passing image build -> Docker file was set up to first build and then move to minimal environment, optimized for small image size.
- speed up build times for faster deployment -> For faster build times linker was changed to [mold](https://github.com/rui314/mold) and debug infos were split from files.