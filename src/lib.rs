use numpy::PyReadonlyArrayDyn;
use numpy::{PyArray, PyArray1};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

use hi_tension::*;

/// An implemementation of hi-tension/TCP.
///
/// Instances must be produced through either connect() or accept().
#[pyclass]
struct HiTensionSocket {
    stream: TcpStream,
}

#[pymethods]
impl HiTensionSocket {
    ///     Connect to a remote hi-tension/TCP server.
    ///
    ///     Parameters
    ///     ----------
    ///     address: str
    ///         The address and port of the remote server.
    ///
    ///     Returns
    ///     -------
    ///     out: HiTensionSocket
    ///         The resulting HiTensionSocket.
    #[staticmethod]
    #[text_signature = "(address)"]
    fn connect(address: &str) -> PyResult<Self> {
        Ok(HiTensionSocket {
            stream: TcpStream::connect(address)?,
        })
    }

    ///     Accept a remote hi-tension/TCP client connection.
    ///
    ///     Parameters
    ///     ----------
    ///     address: str
    ///         The address to bind.Typically "127.0.0.1:XXXX" where
    ///         XXXX is some port.
    ///     Returns
    ///     -------
    ///     out: HiTensionSocket
    ///         The resulting HiTensionSocket.
    #[staticmethod]
    #[text_signature = "(address)"]
    fn accept(address: &str) -> PyResult<Self> {
        let listener = TcpListener::bind(address)?;
        Ok(HiTensionSocket {
            stream: listener.accept()?.0,
        })
    }

    ///     Send text data to the remote. Data shall be ended with a newline
    ///     '\n'.
    ///
    ///     Parameters
    ///     ----------
    ///     data: bytearray
    ///         Text data to send.
    #[text_signature = "($self, data, /)"]
    fn send(&mut self, data: &[u8]) -> PyResult<()> {
        self.stream.write_all(data)?;
        Ok(())
    }

    ///     Read text data from the remote.
    ///
    ///     Returns
    ///     -------
    ///     out: bytearray
    ///         Text data received.
    #[text_signature = "($self, /)"]
    fn read(&mut self, py: Python) -> PyResult<Py<PyByteArray>> {
        let mut bufstream = BufReader::new(&mut self.stream);
        let mut data = vec![];
        let i = bufstream.read_until(4, &mut data)?;

        Ok(PyByteArray::new(py, &data[..i - 1]).into())
    }

    ///     Send a double precision numpy array as a high tension message to
    ///     the remote. Data may be send in multible packets.
    ///
    ///     Parameters
    ///     ----------
    ///     array: ndarray
    ///         Double precision numpy array to send.
    #[text_signature = "($self, array, /)"]
    fn hisend(&mut self, x: PyReadonlyArrayDyn<f64>) -> PyResult<()> {
        let slice = x.as_slice()?;
        hiwrite(&mut self.stream, slice)?;
        Ok(())
    }

    ///     Read a double precision numpy array as a high tension message from
    ///     the remote.
    ///
    ///     Returns
    ///     -------
    ///     out: ndarray
    ///         Double precision numpy array received.
    #[text_signature = "($self, /)"]
    fn hiread(&mut self, py: Python) -> PyResult<Py<PyArray1<f64>>> {
        let buf = hiread(&mut self.stream)?;
        let array = PyArray::from_vec(py, buf);
        Ok(array.to_owned())
    }

    ///     Signal the end of a high tension message to the remote.
    #[text_signature = "($self, /)"]
    fn hidelimiter(&mut self) -> PyResult<()> {
        hidelimiter(&mut self.stream)?;
        Ok(())
    }
}

#[pymodule]
pub fn hi_tension(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<HiTensionSocket>()
}
