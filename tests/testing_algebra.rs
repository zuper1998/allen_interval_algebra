use allen_interval_algebra::interval::Interval;


#[test]
fn precede_test(){
    let x = Interval::new(0,10);
    let y = Interval::new(3,4);
    let z = Interval::new(15,16);

    assert_eq!(x.precede(y),false);
    assert!(x.precede(z))
}

#[test]
fn meets_test(){
    let x = Interval::new(0,10);
    let y = Interval::new(10,14);
    let z = Interval::new(15,16);

    assert_eq!(x.meet(z),false);
    assert!(x.meet(y))


}

#[test]
fn overlaps_test(){
    let x = Interval::new(0,10);
    let y = Interval::new(9,14);
    let z = Interval::new(15,16);

    assert_eq!(x.overlaps(z),false);
    assert!(x.overlaps(y))


}

#[test]
fn starts_test(){
    let x = Interval::new(0,10);
    let y = Interval::new(9,14);
    let z = Interval::new(0,16);

    assert_eq!(x.starts(y),false);
    assert!(x.starts(z))
}

#[test]
fn during_test(){
    let x = Interval::new(1,10);
    let y = Interval::new(9,14);
    let z = Interval::new(0,16);

    assert_eq!(x.during(y),false);
    assert!(x.during(z))}

#[test]
fn finishes_test(){
    let x = Interval::new(8,10);
    let y = Interval::new(9,14);
    let z = Interval::new(3,10);

    assert_eq!(x.finishes(y),false);
    assert!(x.finishes(z))
}

#[test]
fn equal_test(){
    let x = Interval::new(0,10);
    let y = Interval::new(9,14);
    let z = Interval::new(0,10);

    assert_eq!(x.equal(y),false);
    assert!(x.equal(z))
}