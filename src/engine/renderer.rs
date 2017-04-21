use gfx;
use std::sync::mpsc;

#[derive(Debug)]
pub struct EncoderQueue<D: gfx::Device> {
    pub sender: mpsc::Sender<gfx::Encoder<D::Resources, D::CommandBuffer>>,
    pub receiver: mpsc::Receiver<gfx::Encoder<D::Resources, D::CommandBuffer>>,
}

pub struct DeviceRenderer<D: gfx::Device> {
    queue: EncoderQueue<D>,
}

impl<D: gfx::Device> DeviceRenderer<D> {
    pub fn new<CBF>(count: usize,
                    mut create_command_buffer: CBF)
                    -> (DeviceRenderer<D>, EncoderQueue<D>)
        where CBF: FnMut() -> D::CommandBuffer
    {
        let (a_send, b_recv) = mpsc::channel();
        let (b_send, a_recv) = mpsc::channel();

        for _ in 0..count {
            let encoder = gfx::Encoder::from(create_command_buffer());
            a_send.send(encoder).unwrap();
        }

        (DeviceRenderer {
             queue: EncoderQueue {
                 sender: a_send,
                 receiver: a_recv,
             },
         },
         EncoderQueue {
             sender: b_send,
             receiver: b_recv,
         })
    }

    pub fn draw(&mut self, device: &mut D) -> bool {
        match self.queue.receiver.recv() {
            Ok(mut encoder) => {
                encoder.flush(device);
                match self.queue.sender.send(encoder) {
                    Ok(_) => true,
                    Err(e) => {
                        debug!("Unable to send, receiver hung up: {}", e);
                        false
                    }
                }
            }
            Err(e) => {
                debug!("Unable to receive, sender hung up: {}", e);
                false
            }
        }
    }
}
