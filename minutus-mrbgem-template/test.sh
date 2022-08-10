set -eux

cargo build
CMD=$(cargo metadata --format-version 1 | jq -r .target_directory)/debug/minutus-mrbgem-template

NAME=mruby-example
cd tmp
rm -rf $NAME

MRBGEM_TEMPLATE_DEBUG=true $CMD $NAME
cd $NAME

rake test
