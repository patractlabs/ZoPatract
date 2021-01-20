FROM rustlang/rust:nightly

MAINTAINER JacobEberhardt <jacob.eberhardt@tu-berlin.de>, Thibaut Schaeffer <thibaut@schaeff.fr>

RUN useradd -u 1000 -m zopatract

ENV WITH_LIBSNARK=1

COPY ./scripts/install_libsnark_prerequisites.sh /tmp/
RUN /tmp/install_libsnark_prerequisites.sh

COPY ./scripts/install_solcjs_deb.sh /tmp/
RUN /tmp/install_solcjs_deb.sh

USER zopatract

WORKDIR /home/zopatract

COPY --chown=zopatract:zopatract . ZoPatract
