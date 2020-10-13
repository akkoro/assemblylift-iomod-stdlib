COPY_FROM=/usr/src/assemblylift-iomod-stdlib/target/release/

iomods=(akkoro-aws-dynamodb akkoro-std-crypto)

for iomod in "${iomods[@]}"; do
  docker run --rm --entrypoint cat assemblylift-iomod-stdlib:$GITHUB_SHA $COPY_FROM/$iomod > $HOME/$iomod
  chmod 777 $HOME/$iomod
done
