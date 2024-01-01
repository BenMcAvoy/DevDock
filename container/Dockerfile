FROM debian:bookworm

ENV USERNAME=default
ENV PASSWORD=default

RUN apt-get update \
  && apt-get install -y \
    curl \
    dumb-init \
    git \
    git-lfs \
    btop \
    locales \
    lsb-release \
    man-db \
    nano \
    procps \
    vim \
    wget \
    zsh \
    bash \
    xz-utils \
  && git lfs install \
  && rm -rf /var/lib/apt/lists/*

RUN sed -i "s/# en_GB.UTF-8/en_GB.UTF-8/" /etc/locale.gen \
  && locale-gen

ENV LANG=en_GB.UTF-8

# Create a new user
RUN useradd -ms /usr/bin/zsh $USERNAME
RUN echo "$USERNAME:$PASSWORD" | chpasswd

RUN ARCH="$(dpkg --print-architecture)" \
  && curl -fsSL "https://github.com/boxboat/fixuid/releases/download/v0.6.0/fixuid-0.6.0-linux-$ARCH.tar.gz" | tar -C /usr/local/bin -xzf - \
  && chown root:root /usr/local/bin/fixuid \
  && chmod 4755 /usr/local/bin/fixuid \
  && mkdir -p /etc/fixuid \
  && printf "user: $USERNAME\ngroup: $USERNAME\n" > /etc/fixuid/config.yml

RUN curl -fsSL https://code-server.dev/install.sh | sh

# Allow users to have scripts run on container startup to prepare workspace.
# https://github.com/coder/code-server/issues/5177
ENV ENTRYPOINTD=${HOME}/entrypoint.d

EXPOSE 8080

# This way, if someone sets $DOCKER_USER, docker-exec will still work as
# the uid will remain the same. note: only relevant if -u isn't passed to
# docker-run.
USER 1000
ENV USER=$USERNAME
WORKDIR /home/$USERNAME

COPY /entrypoint.sh /usr/bin/entrypoint.sh
ENTRYPOINT ["/usr/bin/entrypoint.sh", "--bind-addr", "0.0.0.0:8080", "."]
