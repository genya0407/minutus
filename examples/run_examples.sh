set -eux

BASEDIR=$(dirname "$0")

cd $BASEDIR/plane-mruby && cargo clean && cargo run
cd $BASEDIR/custom-mruby && cargo clean && cargo run
cd $BASEDIR/mruby-polars && rake clean test
