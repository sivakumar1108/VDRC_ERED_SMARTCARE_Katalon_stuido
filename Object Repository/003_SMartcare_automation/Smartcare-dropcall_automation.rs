<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Smartcare-dropcall_automation</name>
   <tag></tag>
   <elementGuidId>89eb7aed-2420-4b2f-b197-f674eb3c69af</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;\u003csoapenv:Envelope xmlns:soapenv\u003d\&quot;http://schemas.xmlsoap.org/soap/envelope/\&quot; xmlns:v1\u003d\&quot;http://seq.huawei.com/interface/dsi/v1.1\&quot;\u003e\n\u003csoapenv:RequestHeader\u003e\n\t\u003creqTime\u003e2020-11-21 04:06:01\u003c/reqTime\u003e\n\t\u003c/soapenv:RequestHeader\u003e\n \u003csoapenv:RequestBody\u003e\n\n                   \u003cDropUserList\u003e\n                   \u003cUser\u003e\n                     \u003cstarttime\u003e2020-10-11 10:03:04\u003c/starttime\u003e\n                     \u003cmsisdn\u003e${drop_msisdn}\u003c/msisdn\u003e\n                     \u003cimsi\u003e630010479056591\u003c/imsi\u003e\n                     \u003cimei\u003e358589086784963\u003c/imei\u003e\n                     \u003ccell_id\u003e62001234CED1D\u003c/cell_id\u003e\n                     \u003clayer2id\u003e10\u003c/layer2id\u003e\n                     \u003cpeer_number\u003e827146055\u003c/peer_number\u003e\n                     \u003csrv_type\u003emoc\u003c/srv_type\u003e\n                     \u003crat\u003e2\u003c/rat\u003e\n                   \u003c/User\u003e\n                   \u003c/DropUserList\u003e\n \u003c/soapenv:RequestBody\u003e\n\u003c/soapenv:Envelope\u003e&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
   </httpHeaderProperties>
   <katalonVersion>7.8.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${smartcare_url}/SmartCarePlugin/WSDLLoader?group=CALLDROP</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.drop_msisdn</defaultValue>
      <description></description>
      <id>b7189871-6c96-4259-a93f-720112c4f939</id>
      <masked>false</masked>
      <name>drop_msisdn</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.smartcare_url</defaultValue>
      <description></description>
      <id>7202a45c-1e89-4896-9c76-19a1fe623356</id>
      <masked>false</masked>
      <name>smartcare_url</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
