FROM python:3-slim-bookworm as build

ARG BRANCH_NODE=release-1.5.8
ARG BRANCH_CLIENT=release-2.0.0
ARG BRANCH_SIDECAR

RUN apt-get update \
  && apt-get install -y \
  curl \
  build-essential \
  libssl-dev \
  libffi-dev \
  pkg-config \
  make \
  cmake \
  gcc \
  g++ \
  jq \
  git \
  wabt \
  && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="$PATH:/root/.cargo/bin"

SHELL ["/bin/bash", "--login", "-c"]

WORKDIR /app

RUN git clone -b $BRANCH_NODE https://github.com/casper-network/casper-node.git ;
RUN git clone -b $BRANCH_CLIENT https://github.com/casper-ecosystem/casper-client-rs.git ;
RUN git clone -b main https://github.com/casper-network/casper-node-launcher.git ;
RUN if [ -n "$BRANCH_SIDECAR" ]; then \
  git clone https://github.com/casper-network/casper-nctl.git ; \
  git clone -b $BRANCH_SIDECAR https://github.com/casper-network/casper-sidecar.git ; \
  else \
  mkdir -p casper-sidecar/target/release && mkdir casper-sidecar/resources ; \
  touch casper-sidecar/target/release/casper-sidecar ; \
  ln -s casper-node/utils/nctl casper-nctl ;\
  fi

COPY sh/*.sh .
RUN chmod +x ./*.sh

ENV VIRTUAL_ENV=/app/venv
ENV PATH="$VIRTUAL_ENV/bin:$PATH"
RUN ./setup-python.sh >> setup-python_output.txt

RUN if [ -n "$BRANCH_SIDECAR" ]; then \
  ./compile.sh >> compile_output.txt; \
  else \
  ./compile.sh "/app/casper-node/utils/nctl" >> compile_output.txt; \
  fi

# CMD ["/bin/bash", "-ci", "cat compile_output.txt"]

FROM python:3-slim-bookworm as run

ARG BRANCH_SIDECAR
ENV BRANCH_SIDECAR=${BRANCH_SIDECAR}

RUN apt-get update \
  && apt-get install -y \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY sh/*.sh .
RUN chmod +x ./*.sh

ENV VIRTUAL_ENV=/app/venv
ENV PATH="$VIRTUAL_ENV/bin:$PATH"
RUN ./setup-python.sh >> setup-python_output.txt

COPY --from=build /app/casper-node/resources ./casper-node/resources
COPY --from=build /app/casper-node/target/release/casper-node ./casper-node/target/release/casper-node
COPY --from=build /app/casper-client-rs/resources ./casper-client-rs/resources
COPY --from=build /app/casper-client-rs/target/release/casper-client ./casper-client-rs/target/release/casper-client
COPY --from=build /app/casper-node-launcher/resources ./casper-node-launcher/resources
COPY --from=build /app/casper-node-launcher/target/release/casper-node-launcher ./casper-node-launcher/target/release/casper-node-launcher
COPY --from=build /app/casper-sidecar/resources ./casper-sidecar/resources
COPY --from=build /app/casper-sidecar/target/release/casper-sidecar ./casper-sidecar/target/release/casper-sidecar
COPY --from=build /app/casper-nctl ./casper-nctl

RUN if [ -z "$BRANCH_SIDECAR" ]; then \
  mkdir -p ./casper-node/utils ; \
  mv ./casper-nctl ./casper-node/utils/nctl ; \
  ln -s casper-node/utils/nctl casper-nctl ;\
  fi

CMD ["/bin/bash", "-c", "\
  if [ -n \"$BRANCH_SIDECAR\" ]; then \
  /bin/bash -ci \"/app/restart.sh\"; \
  else \
  /bin/bash -ci \"/app/restart.sh /app/casper-node/utils/nctl\"; \
  fi"]

EXPOSE 11101-11105 14101-14105 18101-18105 25101-25105 28101-28105