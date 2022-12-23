use crate::days::internal_common::*;

pub fn day_13_part_1<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let input = get_whole_input_as_string(input)?;
    let mut sum = 0;
    let mut idx = 1;
    parse::parse_and_do_for_packet_pair(&input, |a, b| {
        if a < b {
            sum += idx;
        }
        idx += 1;
        Ok(())
    })?;

    println!("Sum is {}", sum);

    Ok(())
}

pub fn day_13_part_2<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let input = get_whole_input_as_string(input)?;
    let mut packets = parse::parse_and_collect_packets(&input)?;
    packets.push(PacketElement::Int(2));
    packets.push(PacketElement::Int(6));

    let mut indices: Vec<usize> = (0..packets.len()).into_iter().collect();
    let special_index_0 = packets.len() - 2;
    let mut special_index_0_new_pos = 0;
    let special_index_1 = packets.len() - 1;
    let mut special_index_1_new_pos = 0;
    indices.sort_by(|&a, &b| packets[a].cmp(&packets[b]));
    for (new_pos, &index) in indices.iter().enumerate() {
        if index == special_index_0 {
            special_index_0_new_pos = new_pos + 1;
        }
        else if index == special_index_1 {
            special_index_1_new_pos = new_pos + 1;
        }
    }
    let product = special_index_0_new_pos * special_index_1_new_pos;
    println!("Special product is {}", product);

    Ok(())
}

impl std::fmt::Display for PacketElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(x) => write!(f, "{}", x),
            Self::Array(a) => {
                write!(f, "[")?;
                for e in a {
                    e.fmt(f)?;
                }
                write!(f, "]")
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum PacketElement {
    Int(u32),
    Array(Vec<PacketElement>)
}

impl PacketElement {
    fn is_int(&self) -> bool {
        match self {
            Self::Int(_) => true,
            _ => false
        }
    }
}

fn cmp_int_array(a: &PacketElement, b: &PacketElement) -> Option<std::cmp::Ordering> {

    use std::cmp::Ordering;

    let (a_inner, b_inner) = match (a, b) {
        (PacketElement::Int(a), PacketElement::Array(b)) => (a, b),
        _ => panic!("Expected int and array variants")
    };
    
    if b_inner.len() == 0 {
        return Some(Ordering::Greater);
    }
    else {
        let ord = match b_inner[0] {
            PacketElement::Int(x) => a_inner.partial_cmp(&x),
            PacketElement::Array(_) => cmp_int_array(a, &b_inner[0])
        }.unwrap();
        if ord != Ordering::Equal {
            return Some(ord);
        }
        else {
            return 1.partial_cmp(&b_inner.len());
        }
    }
}

impl PartialOrd for PacketElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {

        use std::cmp::Ordering;

        match (self, other) {
            (Self::Int(a), Self::Int(b)) => a.partial_cmp(b),
            (Self::Array(a), Self::Array(b)) => {

                for i in 0..(a.len().min(b.len())) {
                    let ord = a[i].partial_cmp(&b[i]).unwrap();
                    if ord != Ordering::Equal {
                        return Some(ord);
                    }
                }
                
                a.len().partial_cmp(&b.len())

            }
            (a, b) => {
                if a.is_int() {
                    cmp_int_array(a, b)
                }
                else {
                    cmp_int_array(b, a).map(Ordering::reverse)
                }
            }
        }
    }
}

impl Ord for PacketElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

mod parse {
    use crate::days::parse::*;
    use crate::days::parse::nom_goes_brrr::*;

    fn parse_packet<'a, E>(input: &'a str) -> IResult<&'a str, super::PacketElement, E>
    where E: ParseError<&'a str>
    {
        let array_start = tag("[");
        let array_end = tag("]");
        let parse_packet_el_int = |i| {
            match parse_int::<_, E>(i) {
                Ok((i, x)) => Ok((i, super::PacketElement::Int(x))),
                Err(a) => Err(a),
            }
        };

        let (i, _) = array_start(input)?;

        let (i, list) = separated_list0(tag(","), 
            alt((
                parse_packet_el_int,
                parse_packet
            ))
        )(i)?;

        let (i, _) = array_end(i)?;

        Ok((i, super::PacketElement::Array(list)))
    }

    pub(super) fn parse_and_do_for_packet_pair<F>(input: &str, mut func: F) -> super::Result<()>
    where F: FnMut(super::PacketElement, super::PacketElement) -> super::Result<()>
    {
        let mut i = input;
        while i.starts_with("[") {
            let mut parse_packet_line = terminated(parse_packet, newline);
            let packet_a;
            let packet_b;
            (i, packet_a) = make_verbose_error_message(input, parse_packet_line(i))?;
            (i, packet_b) = make_verbose_error_message(input, parse_packet_line(i))?;

            func(packet_a, packet_b)?;

            (i, _) = make_verbose_error_message(input, opt(newline)(i))?;
        }
        Ok(())
    }

    pub(super) fn parse_and_collect_packets(input: &str) -> super::Result<Vec<super::PacketElement>>
    {
        let (_, packets) = make_verbose_error_message(input,
            many0(
                terminated(
                    terminated(parse_packet, newline),
                    opt(newline)
                )
            )(input)
        )?;
        Ok(packets)
    }
}