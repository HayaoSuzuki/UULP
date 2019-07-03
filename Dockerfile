FROM ubuntu:bionic

RUN sed -i '/path-exclude=\/usr\/share\/man\/*/c\#path-exclude=\/usr\/share\/man\/*' /etc/dpkg/dpkg.cfg.d/excludes && \
    apt update && \
    apt install -y build-essential man manpages-dev man-db strace && \
    dpkg -l | grep ^ii | cut -d' ' -f3 | xargs apt install -y --reinstall && \
    catman > /dev/null
CMD ["bash"]
