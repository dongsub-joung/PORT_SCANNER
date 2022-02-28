use std::time::Instant;

#[derive(Debug)]
pub struct Benchmark {
    named_timers: Vec<NamedTimer>,
}

impl Benchmark {
    pub fn init() -> Self {
        Self {
            named_timers: Vec::new(),
        }
    }

    pub fn push(&mut self, timer: NamedTimer){
        self.named_timers.push(timer);
    }

    pub fn summary(&self) -> String {
        let mut summary= String::from("\n RustScan Benchmark Summary");

        for timer in &self.named_timers{
            if timer.start.is_some() && timer.end.is_some(){
                let runtime_secs = timer
                    .end
                    .unwrap()
                    .saturating_druations_since(timer.start.unwrap())
                    .as_secs_f32();
                
                summary.push_str(&format!("\n{0: < 10} | {1: <10}s", timer.name, runtime_secs))
            }
        }

        summary
    }
}

#[derive(Debug)]
pub struct NamedTimer{
    name: &'static str,
    statrt: Option<Instatnt>,
    end: Option<Instant>,
}

impl NamedTimer {
    pub fn start(name: &'static str) -> Self {
        Self {
            name,
            start: Some(Instant::now()),
            end: None,
        }
    }
    pub fn end(&mut self) {
        self.end= Some(Instant::now());
    }    
}

#[test]
fn benchmark(){
    let mut benchmarks= Benchmark::init();
    let mut test_timer= NamedTimer::start("test");
    std::thread::sleep(std::time::Duration::from_millies(100));
    test_timer.end();
    benchmarks.push(test_timer);
    benchmarks.push(NameTimer::start("Only_start"));
    assert!(benchmarks
        .summary()
        .contains("RustScan Benchmark Summary \n test | 0."));
    assert_ne!(benchmarks.summary().contains("only_start"), true);
}