use tornarec::arm7tdmi::cpu::Arm7TDMI;

#[test]
fn test_eval_suffix() {
    let cpu = Arm7TDMI::new();

    assert!(cpu.eval_suffix("EQ").unwrap());

    assert!(!cpu.eval_suffix("NE").unwrap());

    assert!(!cpu.eval_suffix("CS").unwrap());
    assert!(!cpu.eval_suffix("HS").unwrap());

    assert!(cpu.eval_suffix("CC").unwrap());
    assert!(cpu.eval_suffix("LO").unwrap());

    assert!(!cpu.eval_suffix("MI").unwrap());
    assert!(cpu.eval_suffix("PL").unwrap());

    assert!(!cpu.eval_suffix("VS").unwrap());
    assert!(cpu.eval_suffix("VC").unwrap());

    assert!(!cpu.eval_suffix("HI").unwrap());
    assert!(cpu.eval_suffix("LS").unwrap());

    assert!(cpu.eval_suffix("GE").unwrap());

    assert!(!cpu.eval_suffix("LT").unwrap());

    assert!(cpu.eval_suffix("GT").unwrap());
    assert!(cpu.eval_suffix("LE").unwrap());
    assert!(cpu.eval_suffix("AL").unwrap());
    assert!(cpu.eval_suffix("NV").unwrap());

    assert!(cpu.eval_suffix("NO").is_err());
}
