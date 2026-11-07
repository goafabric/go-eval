# the static linux binary is cross-compiled on the host first, see build-image.sh
FROM scratch
COPY dist/callee-service /callee-service
USER 65532:65532
EXPOSE 50900
ENTRYPOINT ["/callee-service"]
