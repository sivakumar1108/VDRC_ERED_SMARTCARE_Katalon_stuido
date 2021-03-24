<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Queryprofile</name>
   <tag></tag>
   <elementGuidId>c202a1a4-f5af-4746-b6f4-a8693e210598</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>7.8.2</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soap:Envelope xmlns:soap=&quot;http://www.w3.org/2003/05/soap-envelope&quot;
        xmlns:quer=&quot;http://www.huawei.com/bme/cbsinterface/querymgr&quot; xmlns:com=&quot;http://www.huawei.com/bme/cbsinterface/common&quot;
        xmlns:quer1=&quot;http://www.huawei.com/bme/cbsinterface/query&quot;>
        &lt;soap:Header />
        &lt;soap:Body>
                &lt;quer:QuerySubCosIDRequestMsg>
                        &lt;RequestHeader>
                                &lt;com:CommandId>QuerySubCosID&lt;/com:CommandId>
                                &lt;com:Version>1&lt;/com:Version>
                                &lt;com:TransactionId>Null&lt;/com:TransactionId>
                                &lt;com:SequenceId>1&lt;/com:SequenceId>
                                &lt;com:RequestType>Event&lt;/com:RequestType>
                                &lt;com:SerialNo>3421lker32e346dhhg&lt;/com:SerialNo>
                        &lt;/RequestHeader>
                        &lt;QuerySubCosIDRequest>
                                &lt;quer1:MSISDN>${RAM_MSISDN}&lt;/quer1:MSISDN>
                        &lt;/QuerySubCosIDRequest>
                &lt;/quer:QuerySubCosIDRequestMsg>
        &lt;/soap:Body>
&lt;/soap:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>${RAM_Query_Endpoint}</soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.RAM_Query_Endpoint</defaultValue>
      <description></description>
      <id>0869f5ca-6cbc-4a14-bc47-1c667c9c4f22</id>
      <masked>false</masked>
      <name>RAM_Query_Endpoint</name>
   </variables>
   <variables>
      <defaultValue>'825371945'</defaultValue>
      <description></description>
      <id>8fc9f068-672c-4696-b24f-e0a795fda970</id>
      <masked>false</masked>
      <name>RAM_MSISDN</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress>file:/D:/shiva/Testing/025-RAM_DEBT_campaign/SoapUI_WSDL/Queryprofile.wsdl.xml</wsdlAddress>
</WebServiceRequestEntity>
