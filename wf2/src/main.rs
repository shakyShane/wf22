fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let tasks = wf2_core::Wf2::from_args(args.into_iter());
}
